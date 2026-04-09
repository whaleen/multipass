use camino::Utf8PathBuf;
use multipass_core::{MultipassConfig, RoomConfig};
use tempfile::TempDir;

#[test]
fn save_and_load_round_trip() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let dir = Utf8PathBuf::from_path_buf(temp.path().to_path_buf()).unwrap();
    let config = MultipassConfig {
        wing: "alpha".into(),
        rooms: vec![
            RoomConfig {
                name: "src".into(),
                description: Some("source".into()),
                keywords: vec!["src".into()],
            },
            RoomConfig {
                name: "general".into(),
                description: None,
                keywords: Vec::new(),
            },
        ],
    };

    config.save(&dir)?;
    let loaded = MultipassConfig::load(&dir)?;

    assert_eq!(loaded.wing, "alpha");
    assert_eq!(loaded.rooms.len(), 2);
    assert_eq!(loaded.rooms[0].name, "src");
    assert_eq!(loaded.rooms[0].keywords, vec!["src"]);
    Ok(())
}
