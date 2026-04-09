#[derive(Debug, Clone)]
pub struct AaakBrief {
    pub ship: String,
    pub total_records: usize,
    pub wings: Vec<(String, usize)>,
    pub rooms: Vec<(String, usize)>,
    pub recent_records: Vec<AaakRecentRecord>,
}

#[derive(Debug, Clone)]
pub struct AaakRecentRecord {
    pub wing: String,
    pub room: String,
    pub preview: String,
}

impl AaakBrief {
    pub fn render(&self) -> String {
        let mut lines = Vec::new();
        lines.push("AAAK wake-up".to_string());
        lines.push(format!("ship: {}", self.ship));
        lines.push(format!(
            "records: {} across {} wing(s) and {} room(s)",
            self.total_records,
            self.wings.len(),
            self.rooms.len()
        ));

        if !self.wings.is_empty() {
            lines.push("wings:".to_string());
            for (wing, count) in &self.wings {
                lines.push(format!("  - {wing}: {count}"));
            }
        }

        if !self.rooms.is_empty() {
            lines.push("rooms:".to_string());
            for (room, count) in &self.rooms {
                lines.push(format!("  - {room}: {count}"));
            }
        }

        if self.recent_records.is_empty() {
            lines.push("recent records: none".to_string());
        } else {
            lines.push("recent records:".to_string());
            for record in &self.recent_records {
                lines.push(format!(
                    "  - [{}/{}] {}",
                    record.wing, record.room, record.preview
                ));
            }
        }

        lines.join("\n")
    }
}
