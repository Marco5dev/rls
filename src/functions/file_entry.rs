use std::path::PathBuf;
use std::fs::Metadata;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub metadata: Metadata,
}
