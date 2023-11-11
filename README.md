# Material Converter

Assemble found PBR materials into a Bevy-compatible format.

```
Tool for creating array textures from found materials.

Usage: material-converter <COMMAND>

Commands:
  guess-input          Use heuristics to guess the material attribute of each file
  convert-images       Convert images to the desired format
  make-array-material  Combine multiple materials into an array material
  feeling-lucky        guess-input, convert-images, then make-array-material
  help                 Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```