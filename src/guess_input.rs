use super::MaterialAttribute;
use anyhow::Context;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

const SUBSTRING_MAP: [(&str, MaterialAttribute); 12] = [
    ("ao", MaterialAttribute::AmbientOcclusion),
    ("ambient", MaterialAttribute::AmbientOcclusion),
    ("occlusion", MaterialAttribute::AmbientOcclusion),
    ("norm", MaterialAttribute::Normal),
    ("albedo", MaterialAttribute::Albedo),
    ("base", MaterialAttribute::Albedo),
    ("color", MaterialAttribute::Albedo),
    ("rough", MaterialAttribute::Roughness),
    ("metal", MaterialAttribute::Metallic),
    ("depth", MaterialAttribute::Depth),
    ("height", MaterialAttribute::Depth),
    ("emissi", MaterialAttribute::Emissive),
];

pub fn guess_input(input_dir: &Path, output_file: &Path) -> anyhow::Result<()> {
    let mut guessed_attrs = HashSet::<MaterialAttribute>::new();
    let mut guesses = Vec::<(MaterialAttribute, PathBuf)>::new();
    for entry in std::fs::read_dir(input_dir).with_context(|| format!("{input_dir:?}"))? {
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

        let mut match_counts = HashMap::<_, u32>::default();
        for (substr, attr) in SUBSTRING_MAP {
            if !name.to_string_lossy().to_lowercase().contains(substr) {
                continue;
            }

            let count = match_counts.entry(attr).or_default();
            *count += 1;
        }

        // Choose the attribute with the most matches.
        let mut max_matches = 0;
        let mut matching_attr = None;
        for (attr, count) in match_counts {
            if count == 0 {
                continue;
            }
            if max_matches > 0 {
                eprintln!("Found multiple matches for {:?}", path);
            }
            if count > max_matches {
                max_matches = count;
                matching_attr = Some(attr);
            }
        }
        if let Some(attr) = matching_attr {
            if !guessed_attrs.insert(attr) {
                eprintln!(
                    "Guessing {:?} again, requires manual resolution in the output file",
                    attr
                );
            }
            guesses.push((attr, path.clone()));
        } else {
            eprintln!("Failed to guess attribute for {:?}", path);
        }
    }

    let s = ron::ser::to_string_pretty(&guesses, Default::default())?;
    std::fs::write(output_file, s)?;

    Ok(())
}
