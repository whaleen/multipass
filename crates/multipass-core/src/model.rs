use camino::Utf8PathBuf;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Record {
    pub id: String,
    pub wing: String,
    pub room: String,
    pub corridor: Option<String>,
    pub source_path: Option<Utf8PathBuf>,
    pub source_mtime: Option<DateTime<Utc>>,
    pub added_by: String,
    pub created_at: DateTime<Utc>,
    pub content: String,
    pub metadata: RecordMetadata,
}

#[derive(Debug, Clone)]
pub struct RecordMetadata {
    pub chunk_index: u32,
}
