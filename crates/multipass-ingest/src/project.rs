use anyhow::Context;
use camino::{Utf8Path, Utf8PathBuf};
use chrono::{DateTime, Utc};
use multipass_core::{MultipassConfig, Record, RecordMetadata, RoomConfig};
use uuid::Uuid;
use walkdir::WalkDir;

use crate::chunk::chunk_text;

pub fn default_project_config(project_dir: &Utf8Path) -> MultipassConfig {
    let mut rooms = Vec::new();
    if let Ok(entries) = std::fs::read_dir(project_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with('.') {
                    continue;
                }
                rooms.push(RoomConfig {
                    name: name.clone(),
                    description: Some(format!("Files from {name}/")),
                    keywords: vec![name.clone()],
                });
            }
        }
    }
    rooms.push(RoomConfig {
        name: "general".to_string(),
        description: Some("Files that don't fit other rooms".to_string()),
        keywords: Vec::new(),
    });

    MultipassConfig {
        wing: project_dir
            .file_name()
            .map(|s| s.replace('-', "_"))
            .unwrap_or_else(|| "default".to_string()),
        rooms,
    }
}

pub fn ingest_project(
    project_dir: &Utf8Path,
    config: &MultipassConfig,
) -> anyhow::Result<Vec<Record>> {
    let mut records = Vec::new();
    for entry in WalkDir::new(project_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = Utf8PathBuf::from_path_buf(entry.path().to_path_buf())
            .map_err(|_| anyhow::anyhow!("non-utf8 path"))?;
        let rel = path
            .strip_prefix(project_dir)
            .map(|p| p.to_string())
            .unwrap_or_else(|_| path.to_string());
        if should_skip(&rel) {
            continue;
        }

        let content =
            std::fs::read_to_string(&path).with_context(|| format!("reading {}", path))?;
        if content.trim().is_empty() {
            continue;
        }

        let room = detect_room(&rel, config);
        let source_mtime = std::fs::metadata(&path)
            .ok()
            .and_then(|m| m.modified().ok())
            .map(DateTime::<Utc>::from);

        for (chunk_index, chunk) in chunk_text(&content, 1200, 120).into_iter().enumerate() {
            records.push(Record {
                id: Uuid::new_v4().to_string(),
                wing: config.wing.clone(),
                room: room.clone(),
                corridor: None,
                source_path: Some(path.clone()),
                source_mtime,
                added_by: "multipass-rs".to_string(),
                created_at: Utc::now(),
                content: chunk,
                metadata: RecordMetadata {
                    chunk_index: chunk_index as u32,
                },
            });
        }
    }
    Ok(records)
}

fn should_skip(rel: &str) -> bool {
    rel.starts_with(".git/")
        || rel.starts_with(".venv/")
        || rel == "multipass.yaml"
        || rel.ends_with(".pyc")
        || rel.contains("__pycache__")
}

fn detect_room(rel: &str, config: &MultipassConfig) -> String {
    for room in &config.rooms {
        for keyword in &room.keywords {
            if rel.contains(keyword) {
                return room.name.clone();
            }
        }
    }
    if let Some(head) = rel.split('/').next() {
        for room in &config.rooms {
            if room.name == head {
                return room.name.clone();
            }
        }
    }
    "general".to_string()
}
