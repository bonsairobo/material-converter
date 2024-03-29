{
  description = "NixOS environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShell.${system} = with pkgs;
      mkShell {
        buildInputs = with pkgs; [
          clang
          # Replace llvmPackages with llvmPackages_X, where X is the latest
          # LLVM version (at the time of writing, 16)
          llvmPackages_16.bintools
          mold
          pkg-config
          rustup
          yq-go
        ];

        ###
        ## Rust Toolchain Setup
        ###

        shellHook = ''
          export RUSTC_VERSION=$(yq ".toolchain.channel" rust-toolchain.toml)
          export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
          export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
          rustup component add rust-analyzer
        '';

        ###
        ## Rust Bindgen Setup
        ###

        # So bindgen can find libclang.so
        LIBCLANG_PATH = pkgs.lib.makeLibraryPath [pkgs.llvmPackages_16.libclang.lib];
        # Add headers to bindgen search path
        BINDGEN_EXTRA_CLANG_ARGS =
          # Includes with normal include path
          (builtins.map (a: ''-I"${a}/include"'') [
            # add dev libraries here (e.g. pkgs.libvmi.dev)
            pkgs.glibc.dev
          ])
          # Includes with special directory paths
          ++ [
            ''-I"${pkgs.llvmPackages_16.libclang.lib}/lib/clang/${pkgs.llvmPackages_16.libclang.version}/include"''
            ''-I"${pkgs.glib.dev}/include/glib-2.0"''
            ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
          ];
      };
  };
}
