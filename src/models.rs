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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DuplicatesGroup {
    pub items: Vec<ImageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeduplicationMetadata {
    pub directory_path: PathBuf,
    pub threshold: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeduplicationReport {
    pub metadata: DeduplicationMetadata,
    pub groups: Vec<DuplicatesGroup>,
    pub total_duplicates: usize,
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

        let total_duplicates: usize =
            groups.iter().map(|g| g.items.len()).sum::<usize>() - groups.len();

        DeduplicationReport {
            metadata,
            groups,
            total_duplicates,
        }
    }
}

impl fmt::Display for DeduplicationReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Deduplication Report:")?;
        writeln!(
            f,
            "Directory path: {}",
            self.metadata.directory_path.display()
        )?;
        writeln!(f, "Similarity threshold: {}", self.metadata.threshold)?;
        writeln!(f, "Number of duplicate groups: {}", self.groups.len())?;
        writeln!(f, "Total number of duplicates: {}", self.total_duplicates)?;
        Ok(())
    }
}

#[allow(unused_imports)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_deserialize_dedupe_report() {
        let hash: ImageHash = ImageHash::from_base64("DAIDBwMHAf8").unwrap();
        let image = ImageInfo {
            path: PathBuf::from("/path/to/image.jpg"),
            hash,
        };

        let report = DeduplicationReport {
            metadata: DeduplicationMetadata {
                directory_path: PathBuf::from("/path/to/directory"),
                threshold: 10,
            },
            groups: vec![DuplicatesGroup {
                items: vec![image.clone()],
            }],
            total_duplicates: 1,
        };

        // Serialize and then deserialize the report
        let serialized = serde_json::to_string(&report).unwrap();
        let deserialized: DeduplicationReport = serde_json::from_str(&serialized).unwrap();
        assert_eq!(report, deserialized);
    }
}
