use super::{MaterialAttribute, MaterialFormat};

use image::{DynamicImage, GenericImageView, Rgb, RgbImage};
use std::fs::File;
use std::path::PathBuf;

pub fn convert_images(
    assignment_file: &PathBuf,
    desired_material_format: &MaterialFormat,
    output_directory: &PathBuf,
) -> std::io::Result<()> {
    match desired_material_format {
        MaterialFormat::AmethystPbr => {
            convert_images_to_amethyst_format(assignment_file, output_directory)
        }
    }
}

fn convert_images_to_amethyst_format(
    assignment_file: &PathBuf,
    output_directory: &PathBuf,
) -> std::io::Result<()> {
    if !output_directory.exists() {
        std::fs::create_dir(output_directory)?;
    }

    let assignments: Vec<(MaterialAttribute, String)> =
        ron::de::from_reader(File::open(assignment_file)?).unwrap();

    let mut metadata = Vec::new();
    for (attr, path) in assignments.iter() {
        let img = image::open(path).unwrap();
        let new_name = attr.canonical_name();
        let converted_img = match attr.convert_image(&img) {
            Some(i) => i,
            None => {
                println!("Skipping {:?}; Depth format not supported yet", path);
                continue;
            }
        };
        converted_img
            .save(output_directory.join(new_name).with_extension("png"))
            .unwrap();
        metadata.push((*attr, path.clone(), img.dimensions()));
    }

    combine_metal_and_rough_amethyst(&assignments).map(|img| {
        let path = output_directory
            .join(MaterialAttribute::MetallicRoughness.canonical_name())
            .with_extension("png");
        metadata.push((
            MaterialAttribute::MetallicRoughness,
            path.to_str().unwrap().to_string(),
            img.dimensions(),
        ));

        img.save(path).unwrap()
    });

    std::fs::write(
        output_directory.join("metadata").with_extension("ron"),
        ron::ser::to_string_pretty(&metadata, Default::default()).unwrap(),
    )?;

    Ok(())
}

fn combine_metal_and_rough_amethyst(
    assignments: &[(MaterialAttribute, String)],
) -> Option<DynamicImage> {
    let metal = open_attribute(&assignments, MaterialAttribute::Metallic);
    let rough = open_attribute(&assignments, MaterialAttribute::Roughness);

    metal.and_then(|m| {
        rough
            .map(|r| {
                if m.dimensions() != r.dimensions() {
                    return None;
                }

                // Write the metal and rough grayscale values into the blue and green channels.
                let mut metal_rough = RgbImage::new(m.width(), m.height());
                let metal_gray = m.to_luma();
                let rough_gray = r.to_luma();
                for (x, y, pixel) in metal_rough.enumerate_pixels_mut() {
                    *pixel = Rgb([
                        0,
                        rough_gray.get_pixel(x, y).0[0],
                        metal_gray.get_pixel(x, y).0[0],
                    ]);
                }

                Some(DynamicImage::ImageRgb8(metal_rough))
            })
            .flatten()
    })
}

fn open_attribute(
    assignments: &[(MaterialAttribute, String)],
    open_attr: MaterialAttribute,
) -> Option<DynamicImage> {
    find_attribute_path(assignments, open_attr).map(|path| image::open(path).unwrap())
}

fn find_attribute_path(
    assignments: &[(MaterialAttribute, String)],
    find_attr: MaterialAttribute,
) -> Option<&String> {
    assignments.iter().find_map(
        |(attr, path)| {
            if *attr == find_attr {
                Some(path)
            } else {
                None
            }
        },
    )
}
