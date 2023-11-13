use super::{convert_images, guess_input, make_array_material, MaterialFormat};
use crate::TextureFormat;
use std::path::{Path, PathBuf};

pub fn feeling_lucky(
    input_directories: &[PathBuf],
    material_format: MaterialFormat,
    texture_format: TextureFormat,
    output_directory: &Path,
) -> anyhow::Result<()> {
    let mut converted_input_dirs = Vec::new();
    for input_dir in input_directories {
        let guesses_path = input_dir.join("guesses").with_extension("ron");
        guess_input(input_dir, &guesses_path)?;
        let output_dir_path = input_dir.with_extension("converted");
        convert_images(
            &guesses_path,
            material_format,
            // Only PNG supported for intermediate conversions.
            TextureFormat::Png,
            &output_dir_path,
        )?;
        converted_input_dirs.push(output_dir_path);
    }
    make_array_material(&converted_input_dirs, texture_format, output_directory)?;

    Ok(())
}
