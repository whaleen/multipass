use camino::Utf8PathBuf;
use multipass_ingest::{default_project_config, ingest_project};
use tempfile::TempDir;

#[test]
fn default_config_includes_visible_directories_and_general() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    std::fs::create_dir_all(temp.path().join("src"))?;
    std::fs::create_dir_all(temp.path().join(".git"))?;
    std::fs::create_dir_all(temp.path().join("docs"))?;
    let project_dir = Utf8PathBuf::from_path_buf(temp.path().to_path_buf()).unwrap();

    let config = default_project_config(&project_dir);
    let room_names = config
        .rooms
        .iter()
        .map(|room| room.name.as_str())
        .collect::<Vec<_>>();

    assert!(room_names.contains(&"src"));
    assert!(room_names.contains(&"docs"));
    assert!(room_names.contains(&"general"));
    assert!(!room_names.contains(&".git"));
    Ok(())
}

#[test]
fn ingest_assigns_files_to_detected_rooms() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    std::fs::create_dir_all(temp.path().join("src"))?;
    std::fs::write(temp.path().join("src/auth.rs"), "pub fn login() {}")?;
    std::fs::write(temp.path().join("README.md"), "general docs")?;

    let project_dir = Utf8PathBuf::from_path_buf(temp.path().to_path_buf()).unwrap();
    let config = default_project_config(&project_dir);
    let records = ingest_project(&project_dir, &config)?;

    assert!(records.iter().any(|record| record.room == "src"));
    assert!(records.iter().any(|record| record.room == "general"));
    Ok(())
}
