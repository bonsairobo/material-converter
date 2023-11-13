use crate::TextureFormat;

use super::{MaterialAttribute, MaterialFormat};
use anyhow::Context;
use image::{DynamicImage, GenericImageView, Rgb, RgbImage};
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn convert_images(
    assignment_file: &Path,
    material_format: MaterialFormat,
    texture_format: TextureFormat,
    output_directory: &Path,
) -> anyhow::Result<()> {
    match material_format {
        MaterialFormat::BevyPbr => {
            convert_images_to_bevy_pbr(assignment_file, texture_format, output_directory)
        }
    }
}

fn convert_images_to_bevy_pbr(
    assignment_file: &Path,
    texture_format: TextureFormat,
    output_directory: &Path,
) -> anyhow::Result<()> {
    std::fs::create_dir_all(output_directory)?;

    let assignments: Vec<(MaterialAttribute, PathBuf)> = ron::de::from_reader(
        File::open(assignment_file)
            .with_context(|| assignment_file.to_string_lossy().into_owned())?,
    )?;

    let mut metadata = Vec::new();
    for (attr, path) in &assignments {
        let img = image::open(path).with_context(|| format!("{path:?}"))?;
        let Some(converted_img) = attr.convert_image(&img) else {
            if attr == &MaterialAttribute::Depth {
                eprintln!("Skipping {:?}; Depth format not supported yet", path);
            }
            continue;
        };
        let new_name = attr.canonical_name();
        let TextureFormat::Png = texture_format;
        let new_path = output_directory.join(new_name).with_extension("png");
        converted_img.save(new_path)?;
        metadata.push((*attr, path.clone(), img.dimensions()));
    }

    if let Some(img) = combine_metal_blue_rough_green(&assignments)? {
        let img_path = output_directory
            .join(MaterialAttribute::MetallicRoughness.canonical_name())
            .with_extension("png");
        img.save(&img_path)?;
        metadata.push((
            MaterialAttribute::MetallicRoughness,
            img_path,
            img.dimensions(),
        ));
    }

    let meta_path = output_directory.join("metadata").with_extension("ron");
    std::fs::write(
        &meta_path,
        ron::ser::to_string_pretty(&metadata, Default::default())?,
    )
    .with_context(|| format!("{meta_path:?}"))?;

    Ok(())
}

/// Write the metal and rough grayscale values into the blue and green channels.
fn combine_metal_blue_rough_green(
    assignments: &[(MaterialAttribute, PathBuf)],
) -> anyhow::Result<Option<DynamicImage>> {
    let metal = open_attribute(assignments, MaterialAttribute::Metallic)?;
    let rough = open_attribute(assignments, MaterialAttribute::Roughness)?;

    let (Some(metal), Some(rough)) = (metal, rough) else {
        return Ok(None);
    };

    if metal.dimensions() != rough.dimensions() {
        return Ok(None);
    }

    let mut metal_rough = RgbImage::new(metal.width(), metal.height());
    let metal_gray = metal.to_luma8();
    let rough_gray = rough.to_luma8();
    for (x, y, pixel) in metal_rough.enumerate_pixels_mut() {
        *pixel = Rgb([
            0,
            rough_gray.get_pixel(x, y).0[0],
            metal_gray.get_pixel(x, y).0[0],
        ]);
    }

    Ok(Some(DynamicImage::ImageRgb8(metal_rough)))
}

fn open_attribute(
    assignments: &[(MaterialAttribute, PathBuf)],
    open_attr: MaterialAttribute,
) -> anyhow::Result<Option<DynamicImage>> {
    assignments
        .iter()
        .find_map(|(attr, path)| (*attr == open_attr).then_some(path))
        .map(|path| image::open(path).with_context(|| format!("{path:?}")))
        .transpose()
}
