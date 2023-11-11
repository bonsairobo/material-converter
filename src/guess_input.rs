use super::MaterialAttribute;
use ron::ser::to_string_pretty;
use std::collections::HashSet;
use std::path::PathBuf;
use std::{fs::File, io::Write};

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

pub fn guess_input(input_directory: &PathBuf, output_file: &PathBuf) -> std::io::Result<()> {
    // Make the guesses based on substring matching.
    let mut guessed_attrs = HashSet::<MaterialAttribute>::new();
    let mut guesses = Vec::<(MaterialAttribute, String)>::new();
    for entry in std::fs::read_dir(input_directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name() {
                let mut found_match = false;
                for (substr, attr) in SUBSTRING_MAP.iter() {
                    if name.to_str().unwrap().to_lowercase().contains(substr) {
                        if guessed_attrs.contains(attr) {
                            println!(
                                "Guessing {:?} again, requires manual resolution in the output file",
                                attr
                            );
                        } else {
                            guessed_attrs.insert(*attr);
                        }
                        if found_match {
                            println!("Found multiple matches for {:?}", path);
                        }
                        guesses.push((*attr, path.to_string_lossy().to_string()));
                        found_match = true;
                    }
                }

                if !found_match {
                    println!("Failed to guess attribute for {:?}", path);
                }
            }
        }
    }

    // Write the results to the output RON file.
    let s = to_string_pretty(&guesses, Default::default()).unwrap();
    File::create(output_file)?.write_all(s.as_bytes())?;

    Ok(())
}
