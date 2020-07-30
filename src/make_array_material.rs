use super::MaterialAttribute;

use image::GenericImage;
use std::fs::File;
use std::path::PathBuf;

pub fn make_array_material(
    input_directories: &[PathBuf],
    output_directory: &PathBuf,
) -> std::io::Result<()> {
    if !output_directory.exists() {
        std::fs::create_dir(output_directory)?;
    }

    // All of the metadata has to match, so we'll just take that of the first one.
    let first_dir = &input_directories[0];
    let metadata: Vec<(MaterialAttribute, String, (u32, u32))> = ron::de::from_reader(File::open(
        first_dir.join("metadata").with_extension("ron"),
    )?)
    .unwrap();

    let num_layers = input_directories.len();

    for (attr, _path, (width, height)) in metadata.iter() {
        let mut concat_img = attr.new_image(*width, height * num_layers as u32);
        for (i, in_dir) in input_directories.iter().enumerate() {
            let start_y = i as u32 * height;
            let img =
                image::open(in_dir.join(attr.canonical_name()).with_extension("png")).unwrap();
            concat_img.copy_from(&img, 0, start_y).unwrap();
        }
        concat_img
            .save(
                output_directory
                    .join(attr.canonical_name())
                    .with_extension("png"),
            )
            .unwrap();
    }

    Ok(())
}
