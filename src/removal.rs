use std::collections::HashSet;
use std::path::PathBuf;

use crate::errors::AppError;
use crate::models::DeduplicationReport;
use serde_json;

/// Run the removal process for the given results file and creates the
/// deduplicated output in the given output directory.
pub fn run(results_file: &str, output_dir: &str) -> Result<(), AppError> {
    let results_file = std::fs::read_to_string(results_file).map_err(
        |_| AppError::FileNotFound(results_file.to_string()),
    )?;
    let results: DeduplicationReport = serde_json::from_str(&results_file).map_err(
        |err| AppError::InvalidDeduplicationReport(err.to_string()),
    )?;

    let output_dir = std::path::Path::new(output_dir);
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir)?;
    }

    // Copy the first image from each group to the output directory (considered the original)
    let mut all_duplicates: HashSet<PathBuf> = HashSet::new();
    for group in results.groups {
        for (index, image) in group.items.iter().enumerate() {
            if index == 0 {
                let file_name = image.path.file_name().unwrap();
                let output_path = output_dir.join(file_name);
                std::fs::copy(&image.path, &output_path)?;
            } else {
                all_duplicates.insert(image.path.clone());
            }
        }
    }
    // Move all non-duplicates to the output directory
    for entry in std::fs::read_dir(&results.metadata.directory_path)? {
        let entry = entry?;
        let path = entry.path();

        if !all_duplicates.contains(&path) {
            let file_name = path.file_name().unwrap();
            let output_path = output_dir.join(file_name);
            std::fs::copy(&path, &output_path)?;
        }
    }

    Ok(())
}
