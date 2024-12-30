use image_hasher::ImageHash;
use serde::{Deserialize, Serialize};
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
