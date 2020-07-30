mod convert_images;
mod feeling_lucky;
mod guess_input;
mod make_array_material;

pub use convert_images::convert_images;
pub use feeling_lucky::feeling_lucky;
pub use guess_input::guess_input;
pub use make_array_material::make_array_material;

use image::DynamicImage;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum MaterialFormat {
    /// Suitable for use with the Amethyst engine's PBR render pass.
    AmethystPbr,
}

impl FromStr for MaterialFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amethyst" | "amethyst-pbr" => Ok(MaterialFormat::AmethystPbr),
            _ => Err(format!("{} is not a supported MaterialFormat", s)),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Hash, Eq, PartialEq, Serialize)]
pub enum MaterialAttribute {
    Albedo,
    AmbientOcclusion,
    Depth, // AKA height
    Emissive,
    Metallic,
    MetallicRoughness,
    Roughness,
    Normal,
}

impl MaterialAttribute {
    fn canonical_name(self: &Self) -> &'static str {
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
            MaterialAttribute::AmbientOcclusion => DynamicImage::new_rgb8(w, h),
            MaterialAttribute::Albedo => DynamicImage::new_rgb8(w, h),
            MaterialAttribute::Depth => DynamicImage::new_rgb8(w, h),
            MaterialAttribute::Emissive => DynamicImage::new_rgb8(w, h),
            MaterialAttribute::Metallic => DynamicImage::new_rgb8(w, h),
            MaterialAttribute::MetallicRoughness => DynamicImage::new_rgb8(w, h),
            MaterialAttribute::Normal => DynamicImage::new_rgb8(w, h),
            MaterialAttribute::Roughness => DynamicImage::new_rgb8(w, h),
        }
    }

    fn convert_image(&self, img: &DynamicImage) -> Option<DynamicImage> {
        match self {
            MaterialAttribute::Normal => Some(DynamicImage::ImageRgb8(img.to_rgb())),
            MaterialAttribute::Albedo => Some(DynamicImage::ImageRgba8(img.to_rgba())),
            MaterialAttribute::AmbientOcclusion => Some(DynamicImage::ImageRgb8(img.to_rgb())),
            _ => None,
        }
    }
}
