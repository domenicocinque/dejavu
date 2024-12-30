use image_hasher::ImageHash;
use serde::{Deserialize, Serialize};
use std::fmt::{self};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageInfo {
    pub path: PathBuf,
    #[serde(
        serialize_with = "crate::serialization::hash_to_base64",
        deserialize_with = "crate::serialization::hash_from_base64"
    )]
    pub hash: ImageHash,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicatesGroup {
    pub items: Vec<ImageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeduplicationMetadata {
    pub directory_path: PathBuf,
    pub threshold: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeduplicationReport {
    pub metadata: DeduplicationMetadata,
    pub groups: Vec<DuplicatesGroup>,
}

impl DeduplicationReport {
    pub fn new(
        directory_path: PathBuf,
        groups: Vec<DuplicatesGroup>,
        duplicate_threshold: u32,
    ) -> Self {
        let metadata = DeduplicationMetadata {
            directory_path,
            threshold: duplicate_threshold,
        };

        DeduplicationReport { metadata, groups }
    }
}

impl fmt::Display for DeduplicationReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Deduplication Report:")?;
        writeln!(
            f,
            "Directory Path: {}",
            self.metadata.directory_path.display()
        )?;
        writeln!(f, "Threshold: {}", self.metadata.threshold)?;
        writeln!(f, "Number of duplicate groups: {}", self.groups.len())?;
        writeln!(
            f,
            "Total number of duplicates: {}",
            self.groups.iter().map(|g| g.items.len()).sum::<usize>()
        )?;
        Ok(())
    }
}
