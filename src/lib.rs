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
    Png,
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
            MaterialAttribute::AmbientOcclusion => "ao",
            MaterialAttribute::Albedo => "albedo",
            MaterialAttribute::Depth => "depth",
            MaterialAttribute::Emissive => "emissive",
            MaterialAttribute::Metallic => "metal",
            MaterialAttribute::MetallicRoughness => "metal_rough",
            MaterialAttribute::Normal => "normal",
            MaterialAttribute::Roughness => "rough",
        }
    }

    fn new_image(&self, w: u32, h: u32) -> DynamicImage {
        match self {
            MaterialAttribute::AmbientOcclusion
            | MaterialAttribute::Depth
            | MaterialAttribute::MetallicRoughness
            | MaterialAttribute::Normal => DynamicImage::new_rgb8(w, h),
            MaterialAttribute::Albedo => DynamicImage::new_rgba8(w, h),
            _ => unimplemented!(),
        }
    }

    fn convert_image(&self, img: &DynamicImage) -> Option<DynamicImage> {
        match self {
            MaterialAttribute::Normal => Some(DynamicImage::ImageRgb8(img.to_rgb8())),
            MaterialAttribute::Albedo => Some(DynamicImage::ImageRgba8(img.to_rgba8())),
            MaterialAttribute::AmbientOcclusion => Some(DynamicImage::ImageRgb8(img.to_rgb8())),
            _ => None,
        }
    }
}
