use super::MaterialAttribute;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

const SUBSTRING_MAP: [(&str, MaterialAttribute); 10] = [
    ("ao", MaterialAttribute::AmbientOcclusion),
    ("ambient", MaterialAttribute::AmbientOcclusion),
    ("occlusion", MaterialAttribute::AmbientOcclusion),
    ("norm", MaterialAttribute::Normal),
    ("albedo", MaterialAttribute::Albedo),
    ("rough", MaterialAttribute::Roughness),
    ("metal", MaterialAttribute::Metallic),
    ("depth", MaterialAttribute::Depth),
    ("height", MaterialAttribute::Depth),
    ("emissi", MaterialAttribute::Emissive),
];

pub fn guess_input(input_directory: &Path, output_file: &Path) -> std::io::Result<()> {
    let mut guessed_attrs = HashSet::<MaterialAttribute>::new();
    let mut guesses = Vec::<(MaterialAttribute, PathBuf)>::new();
    for entry in std::fs::read_dir(input_directory)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            // Only go one level deep.
            eprintln!("Skipping directory {path:?}");
            continue;
        }
        let Some(name) = path.file_name() else {
            continue;
        };

        let mut found_match = false;
        for (substr, attr) in SUBSTRING_MAP.iter() {
            if !name.to_string_lossy().to_lowercase().contains(substr) {
                continue;
            }

            if guessed_attrs.contains(attr) {
                eprintln!(
                    "Guessing {:?} again, requires manual resolution in the output file",
                    attr
                );
            } else {
                guessed_attrs.insert(*attr);
            }
            if found_match {
                eprintln!("Found multiple matches for {:?}", path);
            }
            guesses.push((*attr, path.clone()));
            found_match = true;
        }

        if !found_match {
            eprintln!("Failed to guess attribute for {:?}", path);
        }
    }

    let s = ron::ser::to_string_pretty(&guesses, Default::default()).unwrap();
    std::fs::write(output_file, s)?;

    Ok(())
}
