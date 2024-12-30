use crate::errors::AppError;
use crate::models::{DeduplicationReport, DuplicatesGroup, ImageInfo};
use image_hasher::Hasher;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json;
use std::collections::HashSet;
use std::fs::{self};
use std::path::{Path, PathBuf};

fn get_image_hashes(directory: &Path, hasher: &Hasher) -> Result<Vec<ImageInfo>, AppError> {
    if !directory.is_dir() {
        return Err(AppError::InvalidDirectory(
            directory.to_owned().into_os_string().into_string().unwrap(),
        ));
    }

    let read_dir = fs::read_dir(directory)?;
    let mut image_hashes = Vec::new();

    let spinner: ProgressBar = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template("{spinner:1.cyan/blue} Computing hashes...").unwrap(),
    );
    for entry in read_dir {
        let entry = entry?;
        let path = entry.path();
        if let Ok(image) = image::open(&path) {
            let hash = hasher.hash_image(&image);
            image_hashes.push(ImageInfo { path, hash });
        }
        spinner.tick();
    }

    Ok(image_hashes)
}

fn find_duplicates(images: Vec<ImageInfo>, duplicate_threshold: u32) -> Vec<DuplicatesGroup> {
    let mut groups: Vec<DuplicatesGroup> = Vec::new();
    let mut processed: HashSet<PathBuf> = HashSet::new();

    for (i, image) in images.iter().enumerate() {
        if processed.contains(&image.path) {
            continue;
        }

        let mut current_group: Vec<ImageInfo> = vec![image.clone()];

        for other_image in images.iter().skip(i + 1) {
            if processed.contains(&other_image.path) {
                continue;
            }

            if image.hash.dist(&other_image.hash) < duplicate_threshold {
                current_group.push(other_image.clone());
                processed.insert(other_image.path.clone());
            }
        }

        if current_group.len() > 1 {
            groups.push(DuplicatesGroup {
                items: current_group,
            });
            processed.insert(image.path.clone());
        }
    }

    groups
}

fn save_results(report: DeduplicationReport, path: &Path) -> Result<(), AppError> {
    let contents = serde_json::to_string(&report)?;
    fs::write(path, contents)?;
    println!("Deduplication report saved to {:?}", path);
    Ok(())
}

pub fn run(
    directory: String,
    duplicate_threshold: u32,
    report_filename: &str,
) -> Result<(), AppError> {
    let dir = Path::new(&directory);

    let hasher = image_hasher::HasherConfig::new().to_hasher();
    let image_hashes = get_image_hashes(dir, &hasher)?;
    let duplicates = find_duplicates(image_hashes, duplicate_threshold);
    let output_path = dir.join(report_filename);

    let report = DeduplicationReport::new(dir.to_path_buf(), duplicates, duplicate_threshold);

    println!("{}", report);

    save_results(report, &output_path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbImage;
    use image_hasher::HasherConfig;
    use image_hasher::ImageHash;
    use tempfile::tempdir;

    #[test]
    fn test_get_image_hashes() {
        let dir = tempdir().unwrap();
        let image_path = dir.path().join("image.png");

        let mut image: RgbImage = RgbImage::new(100, 100);
        *image.get_pixel_mut(5, 5) = image::Rgb([255, 255, 255]);
        image.save(&image_path).unwrap();

        let hasher = HasherConfig::new().to_hasher();
        let result = get_image_hashes(dir.path(), &hasher);

        assert!(result.is_ok());
        let image_hashes = result.unwrap();
        assert_eq!(image_hashes.len(), 1);
        assert_eq!(image_hashes[0].path, image_path);
    }

    #[test]
    fn test_find_duplicates() {
        let hash1: ImageHash = ImageHash::from_base64("DAIDBwMHAf8").unwrap();
        let hash2: ImageHash = ImageHash::from_base64("8/JwVtbOVy4").unwrap();
        let hash3 = hash1.clone();

        let image1 = ImageInfo {
            path: PathBuf::from("image1.png"),
            hash: hash1,
        };
        let image2 = ImageInfo {
            path: PathBuf::from("image2.png"),
            hash: hash2,
        };
        let image3 = ImageInfo {
            path: PathBuf::from("image3.png"),
            hash: hash3, // Duplicate of image1
        };

        let images = vec![image1.clone(), image2.clone(), image3.clone()];
        let groups = find_duplicates(images, 10u32);

        assert_eq!(groups.len(), 1, "Expected one group of duplicates");
        assert_eq!(
            groups[0].items.len(),
            2,
            "Expected two items in the duplicate group"
        );
        assert!(groups[0].items.contains(&image1));
        assert!(groups[0].items.contains(&image3));
    }
}
