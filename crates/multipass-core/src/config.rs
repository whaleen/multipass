use camino::Utf8Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipassConfig {
    pub wing: String,
    pub rooms: Vec<RoomConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomConfig {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
}

impl MultipassConfig {
    pub fn load(project_dir: &Utf8Path) -> anyhow::Result<Self> {
        let path = project_dir.join("multipass.yaml");
        let raw = std::fs::read_to_string(&path)?;
        Ok(serde_yaml::from_str(&raw)?)
    }

    pub fn save(&self, project_dir: &Utf8Path) -> anyhow::Result<()> {
        let path = project_dir.join("multipass.yaml");
        let raw = serde_yaml::to_string(self)?;
        std::fs::write(path, raw)?;
        Ok(())
    }
}
