use super::MaterialAttribute;
use crate::{toktx2_array, Ktx2TextureCodec, TextureFormat};
use anyhow::Context;
use image::GenericImage;
use std::fs::File;
use std::path::{Path, PathBuf};

/// When using `TextureFormat::Ktx2*`, it is assumed that images in
/// `input_directories" are already in an input format supported by "toktx".
pub fn make_array_material(
    input_directories: &[PathBuf],
    texture_format: TextureFormat,
    output_directory: &Path,
) -> anyhow::Result<()> {
    std::fs::create_dir_all(output_directory)?;

    // All of the metadata has to match, so we'll just take that of the first one.
    let first_dir = &input_directories[0];
    let meta_path = first_dir.join("metadata").with_extension("ron");
    let metadata: Vec<(MaterialAttribute, PathBuf, (u32, u32))> =
        ron::de::from_reader(File::open(&meta_path).with_context(|| format!("{meta_path:?}"))?)?;

    let num_layers = input_directories.len();

    match texture_format {
        TextureFormat::Png => {
            // Manually create stacked array images.
            for (attr, _path, (width, height)) in metadata {
                let mut concat_img = attr.new_image(width, height * num_layers as u32);
                for (i, in_dir) in input_directories.iter().enumerate() {
                    let start_y = i as u32 * height;
                    let img_path = in_dir.join(attr.canonical_name()).with_extension("png");
                    let img = image::open(&img_path).with_context(|| format!("{img_path:?}"))?;
                    concat_img.copy_from(&img, 0, start_y)?;
                }
                concat_img.save(
                    output_directory
                        .join(attr.canonical_name())
                        .with_extension("png"),
                )?;
            }
        }
        TextureFormat::Ktx2Astc => {
            // PERF: parallelize this
            // Use the "toktx" tool to stack the images for us.
            for (attr, ..) in metadata {
                let input_paths: Vec<PathBuf> = input_directories
                    .iter()
                    // TODO: don't assume PNG here
                    .map(|in_dir| in_dir.join(attr.canonical_name()).with_extension("png"))
                    .collect();
                let output_path = output_directory
                    .join(attr.canonical_name())
                    .with_extension("ktx2");
                toktx2_array(&input_paths, attr, Ktx2TextureCodec::Astc, &output_path)?;
            }
        }
    }

    Ok(())
}
