use camino::Utf8PathBuf;
use chrono::Utc;
use multipass_core::{Record, RecordMetadata, SearchQuery};
use multipass_store::ShipDb;
use tempfile::TempDir;

#[test]
fn replace_wing_records_rebuilds_fts_without_duplicates() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let ship_root = Utf8PathBuf::from_path_buf(temp.path().to_path_buf()).unwrap();
    let mut db = ShipDb::open(&ship_root)?;

    let first = vec![record("rec-1", "alpha", "src", "first auth note")];
    db.replace_wing_records("alpha", &first)?;
    assert_eq!(db.stats()?.total_records, 1);
    assert_eq!(
        db.search(&SearchQuery {
            query: "auth".into(),
            wing: Some("alpha".into()),
            room: None,
            limit: 10,
        })?
        .len(),
        1
    );

    let second = vec![record("rec-2", "alpha", "src", "updated token note")];
    db.replace_wing_records("alpha", &second)?;
    let stats = db.stats()?;
    assert_eq!(stats.total_records, 1);
    assert_eq!(
        db.search(&SearchQuery {
            query: "auth".into(),
            wing: Some("alpha".into()),
            room: None,
            limit: 10,
        })?
        .len(),
        0
    );
    assert_eq!(
        db.search(&SearchQuery {
            query: "token".into(),
            wing: Some("alpha".into()),
            room: None,
            limit: 10,
        })?
        .len(),
        1
    );

    Ok(())
}

#[test]
fn insert_and_delete_record_updates_recent_and_search_state() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let ship_root = Utf8PathBuf::from_path_buf(temp.path().to_path_buf()).unwrap();
    let db = ShipDb::open(&ship_root)?;

    let rec = record("rec-9", "alpha", "notes", "fresh wake up line");
    db.insert_record(&rec)?;

    let recent = db.recent_records(5)?;
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].id, "rec-9");
    assert_eq!(recent[0].preview, "fresh wake up line");

    assert_eq!(
        db.search(&SearchQuery {
            query: "wake".into(),
            wing: Some("alpha".into()),
            room: None,
            limit: 5,
        })?
        .len(),
        1
    );

    assert!(db.delete_record("rec-9")?);
    assert!(!db.delete_record("rec-9")?);
    assert!(db.recent_records(5)?.is_empty());
    assert!(
        db.search(&SearchQuery {
            query: "wake".into(),
            wing: Some("alpha".into()),
            room: None,
            limit: 5,
        })?
        .is_empty()
    );

    Ok(())
}

fn record(id: &str, wing: &str, room: &str, content: &str) -> Record {
    Record {
        id: id.to_string(),
        wing: wing.to_string(),
        room: room.to_string(),
        corridor: None,
        source_path: None,
        source_mtime: None,
        added_by: "test".to_string(),
        created_at: Utc::now(),
        content: content.to_string(),
        metadata: RecordMetadata { chunk_index: 0 },
    }
}
