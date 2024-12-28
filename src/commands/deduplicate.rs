use crate::commands::errors::DuplicationError;
use image_hasher::{Hasher, ImageHash};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{self};
use std::path::{Path, PathBuf};

const DUPLICATE_THRESHOLD: u32 = 10;
const REPORT_FILE_NAME: &str = "dedup_report.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ImageInfo {
    path: PathBuf,
    #[serde(
        serialize_with = "crate::utils::serialization::hash_to_base64",
        deserialize_with = "crate::utils::serialization::hash_from_base64"
    )]
    hash: ImageHash,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ImageDuplicates {
    source: ImageInfo,
    duplicates: Vec<ImageInfo>,
}

impl ImageDuplicates {
    pub fn new(source: ImageInfo, duplicates: Vec<ImageInfo>) -> Option<Self> {
        if duplicates.is_empty() {
            None
        } else {
            Some(ImageDuplicates { source, duplicates })
        }
    }
}

fn get_image_hashes(directory: &Path, hasher: &Hasher) -> Result<Vec<ImageInfo>, DuplicationError> {
    if !directory.is_dir() {
        return Err(DuplicationError::InvalidDirectory(format!(
            "Path '{}' is not a directory",
            directory.display()
        )));
    }

    let read_dir = fs::read_dir(directory)?;
    let mut image_hashes = Vec::new();

    for entry in read_dir {
        let entry = entry?;
        let path = entry.path();
        if let Ok(image) = image::open(&path) {
            let hash = hasher.hash_image(&image);
            image_hashes.push(ImageInfo { path, hash });
        } else {
            eprintln!("Failed to open image: {:?}", path);
        }
    }

    Ok(image_hashes)
}

fn find_duplicates(images: Vec<ImageInfo>) -> Vec<ImageDuplicates> {
    let mut duplicates: Vec<ImageDuplicates> = Vec::new();

    for (i, image) in images.iter().enumerate() {
        let current_duplicates: Vec<ImageInfo> = images
            .iter()
            .skip(i + 1)
            .filter(|other_image| image.hash.dist(&other_image.hash) < DUPLICATE_THRESHOLD)
            .cloned()
            .collect();

        if let Some(image_duplicates) = ImageDuplicates::new(image.clone(), current_duplicates) {
            duplicates.push(image_duplicates);
        }
    }

    duplicates
}

fn save_results(duplicates: Vec<ImageDuplicates>, path: &Path) -> Result<(), DuplicationError> {
    let contents = serde_json::to_string(&duplicates)?;
    fs::write(path, contents)?;
    println!("Deduplication report saved to {:?}", path);
    Ok(())
}

pub fn run(directory: &str) -> Result<(), DuplicationError> {
    let dir = Path::new(directory);
    println!("Deduplicating within {}", directory);

    let hasher = image_hasher::HasherConfig::new().to_hasher();
    let image_hashes = get_image_hashes(dir, &hasher)?;
    let duplicates = find_duplicates(image_hashes);
    let output_path = dir.join(REPORT_FILE_NAME);
    save_results(duplicates, &output_path)?;

    Ok(())
}
