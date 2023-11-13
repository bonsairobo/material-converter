mod convert_images;
mod feeling_lucky;
mod guess_input;
mod make_array_material;

pub use convert_images::convert_images;
pub use feeling_lucky::feeling_lucky;
pub use guess_input::guess_input;
pub use make_array_material::make_array_material;

use clap::ValueEnum;
use image::DynamicImage;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, ValueEnum)]
pub enum MaterialFormat {
    /// Suitable for use with the Bevy engine's PBR render pass.
    ///
    /// - albedo: RGBA8 (sRGB)
    /// - ambient occlusion: Luma8 (linear)
    /// - depth: Luma8 (linear)
    /// - emissive: Luma8 (linear)
    /// - metallic_roughness: RGB8 (linear)
    ///   - only green and blue channels are used
    /// - normal: RGB8 (linear)
    BevyPbr,
}

impl std::fmt::Display for MaterialFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BevyPbr => write!(f, "bevy-pbr"),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, ValueEnum)]
pub enum TextureFormat {
    Ktx2Astc,
    Png,
}

pub enum Ktx2TextureCodec {
    Astc,
}

#[derive(Clone, Copy, Debug, Deserialize, Hash, Eq, PartialEq, Serialize)]
pub enum MaterialAttribute {
    Albedo, // AKA base color
    AmbientOcclusion,
    Depth, // AKA height
    Emissive,
    Metallic,
    MetallicRoughness,
    Normal,
    Roughness,
}

impl MaterialAttribute {
    fn canonical_name(&self) -> &str {
        match self {
            Self::AmbientOcclusion => "ao",
            Self::Albedo => "albedo",
            Self::Depth => "depth",
            Self::Emissive => "emissive",
            Self::Metallic => "metal",
            Self::MetallicRoughness => "metal_rough",
            Self::Normal => "normal",
            Self::Roughness => "rough",
        }
    }

    fn new_image(&self, w: u32, h: u32) -> DynamicImage {
        match self {
            Self::AmbientOcclusion
            | Self::Depth
            | Self::Emissive
            | Self::Metallic
            | Self::Roughness => DynamicImage::new_luma8(w, h),
            Self::MetallicRoughness | Self::Normal => DynamicImage::new_rgb8(w, h),
            Self::Albedo => DynamicImage::new_rgba8(w, h),
        }
    }

    fn convert_image(&self, img: &DynamicImage) -> DynamicImage {
        match self {
            Self::Albedo => DynamicImage::ImageRgba8(img.to_rgba8()),
            Self::AmbientOcclusion
            | Self::Depth
            | Self::Emissive
            | Self::Metallic
            | Self::Roughness => DynamicImage::ImageLuma8(img.to_luma8()),
            Self::MetallicRoughness => DynamicImage::ImageRgb8(img.to_rgb8()),
            Self::Normal => DynamicImage::ImageRgb8(img.to_rgb8()),
        }
    }

    fn toktx2_args(&self) -> Vec<&str> {
        match self {
            Self::Albedo => vec![
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
            Self::AmbientOcclusion
            | Self::Depth
            | Self::Emissive
            | Self::Metallic
            | Self::Roughness => vec![
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
            Self::MetallicRoughness => vec![
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
            Self::Normal => vec![
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
}

pub fn toktx2(
    input_path: &Path,
    attribute: MaterialAttribute,
    codec: Ktx2TextureCodec,
    output_path: &Path,
) -> anyhow::Result<()> {
    let Ktx2TextureCodec::Astc = codec;

    let mut args = attribute.toktx2_args();
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

    let mut args = attribute.toktx2_args();

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
