use std::process::Command;

use tempfile::TempDir;

#[test]
fn init_mine_search_and_wakeup_flow() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let project = temp.path().join("project");
    let ship = temp.path().join("ship");

    std::fs::create_dir_all(project.join("src"))?;
    std::fs::write(project.join("src").join("auth.rs"), "pub fn login() {}\n")?;
    std::fs::write(project.join("README.md"), "# Demo\nAuth path\n")?;

    let init = Command::new(env!("CARGO_BIN_EXE_multipass-rs"))
        .arg("init")
        .arg(&project)
        .output()?;
    assert!(init.status.success());
    let init_stdout = String::from_utf8_lossy(&init.stdout);
    assert!(init_stdout.contains("wing: project"));
    assert!(project.join("multipass.yaml").exists());

    let mine = Command::new(env!("CARGO_BIN_EXE_multipass-rs"))
        .arg("--ship")
        .arg(&ship)
        .arg("mine")
        .arg(&project)
        .output()?;
    assert!(mine.status.success());

    let search = Command::new(env!("CARGO_BIN_EXE_multipass-rs"))
        .arg("--ship")
        .arg(&ship)
        .arg("search")
        .arg("Auth")
        .arg("--limit")
        .arg("5")
        .output()?;
    assert!(search.status.success());
    let search_stdout = String::from_utf8_lossy(&search.stdout);
    assert!(search_stdout.contains("project / general"));

    let wake = Command::new(env!("CARGO_BIN_EXE_multipass-rs"))
        .arg("--ship")
        .arg(&ship)
        .arg("wake-up")
        .output()?;
    assert!(wake.status.success());
    let wake_stdout = String::from_utf8_lossy(&wake.stdout);
    assert!(wake_stdout.contains("AAAK wake-up"));
    assert!(wake_stdout.contains("recent records:"));

    Ok(())
}
