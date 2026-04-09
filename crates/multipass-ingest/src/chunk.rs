pub fn chunk_text(content: &str, chunk_size: usize, overlap: usize) -> Vec<String> {
    if content.trim().is_empty() {
        return Vec::new();
    }
    if content.len() <= chunk_size {
        return vec![content.to_owned()];
    }

    let mut chunks = Vec::new();
    let mut start = 0;
    while start < content.len() {
        let end = (start + chunk_size).min(content.len());
        chunks.push(content[start..end].to_owned());
        if end == content.len() {
            break;
        }
        start = end.saturating_sub(overlap);
    }
    chunks
}
