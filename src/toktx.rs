use crate::{Ktx2TextureCodec, MaterialAttribute};
use std::path::{Path, PathBuf};

pub fn toktx2(
    input_path: &Path,
    attribute: MaterialAttribute,
    codec: Ktx2TextureCodec,
    output_path: &Path,
) -> anyhow::Result<()> {
    let Ktx2TextureCodec::Astc = codec;

    let mut args = material_attribute_args(attribute);
    args.push(output_path.to_str().unwrap());
    args.push(input_path.to_str().unwrap());

    try_run_command("toktx", &args)
}

pub fn toktx2_array(
    input_paths: &[PathBuf],
    attribute: MaterialAttribute,
    codec: Ktx2TextureCodec,
    output_path: &Path,
) -> anyhow::Result<()> {
    let Ktx2TextureCodec::Astc = codec;

    let mut args = material_attribute_args(attribute);

    let num_layers = input_paths.len();
    let num_layers_str = format!("{num_layers}");
    args.push("--layers");
    args.push(&num_layers_str);

    args.push(output_path.to_str().unwrap());
    args.extend(input_paths.iter().map(|p| p.to_str().unwrap()));

    try_run_command("toktx", &args)
}

fn try_run_command(command_name: &str, args: &[&str]) -> anyhow::Result<()> {
    use std::process::Command;

    eprintln!("Running {command_name} with args = {args:?}");

    let out = Command::new(command_name).args(args).output()?;

    if !out.status.success() {
        let mut error_str = format!("{command_name} failed");
        if let Some(code) = out.status.code() {
            error_str.push_str(&format!(" with exit code {code}"));
        }
        if let Ok(stderr_str) = std::str::from_utf8(&out.stderr) {
            if !stderr_str.is_empty() {
                error_str.push_str(&format!("\nSTDERR = {stderr_str}"));
            }
        }
        if let Ok(stdout_str) = std::str::from_utf8(&out.stdout) {
            if !stdout_str.is_empty() {
                error_str.push_str(&format!("\nSTDOUT = {stdout_str}"));
            }
        }
        anyhow::bail!(anyhow::anyhow!(error_str));
    }

    Ok(())
}

fn material_attribute_args(attr: MaterialAttribute) -> Vec<&'static str> {
    match attr {
        MaterialAttribute::Albedo => vec![
            "--2d",
            "--t2",
            "--encode",
            "astc",
            "--genmipmap",
            "--target_type",
            "RGBA",
            "--astc_perceptual",
            "--convert_oetf",
            "srgb",
        ],
        MaterialAttribute::AmbientOcclusion
        | MaterialAttribute::Depth
        | MaterialAttribute::Emissive
        | MaterialAttribute::Metallic
        | MaterialAttribute::Roughness => vec![
            "--2d",
            "--t2",
            "--encode",
            "astc",
            "--genmipmap",
            "--target_type",
            "R",
            "--convert_oetf",
            "linear",
        ],
        MaterialAttribute::MetallicRoughness => vec![
            "--2d",
            "--t2",
            "--encode",
            "astc",
            "--genmipmap",
            "--target_type",
            "RGB",
            "--convert_oetf",
            "linear",
        ],
        MaterialAttribute::Normal => vec![
            "--2d",
            "--t2",
            "--encode",
            "astc",
            "--genmipmap",
            "--target_type",
            "RGB",
            "--astc_perceptual",
            "--convert_oetf",
            "linear",
            "--normalize",
            // This does a weird 2-component (XY) normal encoding where
            // RGB=X A=Y. Bevy only support 2-component normals from 2-
            // channel images.
            // "--normal_mode",
        ],
    }
}
