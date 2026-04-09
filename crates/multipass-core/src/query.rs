#[derive(Debug, Clone, Default)]
pub struct SearchQuery {
    pub query: String,
    pub wing: Option<String>,
    pub room: Option<String>,
    pub limit: usize,
}
