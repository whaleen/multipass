use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SearchHit {
    pub id: String,
    pub wing: String,
    pub room: String,
    pub source_path: Option<String>,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RecentRecord {
    pub id: String,
    pub wing: String,
    pub room: String,
    pub corridor: Option<String>,
    pub source_path: Option<String>,
    pub created_at: String,
    pub preview: String,
}
