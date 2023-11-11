use super::{convert_images, guess_input, make_array_material, MaterialFormat};
use std::path::{Path, PathBuf};

pub fn feeling_lucky(
    input_directories: &[PathBuf],
    output_format: &MaterialFormat,
    output_directory: &Path,
) -> std::io::Result<()> {
    let mut converted_input_dirs = Vec::new();
    for input_dir in input_directories {
        let guesses_path = input_dir.join("guesses").with_extension("ron");
        guess_input(input_dir, &guesses_path)?;
        let output_dir_path = input_dir.with_extension("converted");
        convert_images(&guesses_path, output_format, &output_dir_path)?;
        converted_input_dirs.push(output_dir_path);
    }
    make_array_material(&converted_input_dirs, output_directory)?;

    Ok(())
}
