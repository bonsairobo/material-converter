use super::{MaterialAttribute, MaterialFormat};
use image::{DynamicImage, GenericImageView, Rgb, RgbImage};
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn convert_images(
    assignment_file: &Path,
    output_format: &MaterialFormat,
    output_directory: &Path,
) -> std::io::Result<()> {
    match output_format {
        MaterialFormat::BevyPbr => convert_images_to_bevy_pbr(assignment_file, output_directory),
    }
}

fn convert_images_to_bevy_pbr(
    assignment_file: &Path,
    output_directory: &Path,
) -> std::io::Result<()> {
    std::fs::create_dir_all(output_directory)?;

    let assignments: Vec<(MaterialAttribute, PathBuf)> =
        ron::de::from_reader(File::open(assignment_file)?).unwrap();

    let mut metadata = Vec::new();
    for (attr, path) in &assignments {
        let img = image::open(path).unwrap();
        let Some(converted_img) = attr.convert_image(&img) else {
            eprintln!("Skipping {:?}; Depth format not supported yet", path);
            continue;
        };
        let new_name = attr.canonical_name();
        converted_img
            .save(output_directory.join(new_name).with_extension("png"))
            .unwrap();
        metadata.push((*attr, path.clone(), img.dimensions()));
    }

    if let Some(img) = combine_metal_blue_rough_green(&assignments) {
        let path = output_directory
            .join(MaterialAttribute::MetallicRoughness.canonical_name())
            .with_extension("png");
        img.save(&path).unwrap();
        metadata.push((MaterialAttribute::MetallicRoughness, path, img.dimensions()));
    }

    std::fs::write(
        output_directory.join("metadata").with_extension("ron"),
        ron::ser::to_string_pretty(&metadata, Default::default()).unwrap(),
    )?;

    Ok(())
}

/// Write the metal and rough grayscale values into the blue and green channels.
fn combine_metal_blue_rough_green(
    assignments: &[(MaterialAttribute, PathBuf)],
) -> Option<DynamicImage> {
    let metal = open_attribute(assignments, MaterialAttribute::Metallic)?;
    let rough = open_attribute(assignments, MaterialAttribute::Roughness)?;

    if metal.dimensions() != rough.dimensions() {
        return None;
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

    Some(DynamicImage::ImageRgb8(metal_rough))
}

fn open_attribute(
    assignments: &[(MaterialAttribute, PathBuf)],
    open_attr: MaterialAttribute,
) -> Option<DynamicImage> {
    assignments
        .iter()
        .find_map(|(attr, path)| (*attr == open_attr).then_some(path))
        .map(|path| image::open(path).unwrap())
}
