use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manifest {
    pub package: PackageInfo,
    #[serde(default)]
    pub dependencies: HashMap<String, Dependency>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub authors: Option<Vec<String>>,
    pub description: Option<String>,
    pub entry: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Dependency {
    Git { git: String, tag: Option<String>, branch: Option<String> },
    Path { path: String },
    Version(String), // For future decentralized registry / opm server
}

impl Manifest {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read Omni.toml at {:?}", path.as_ref()))?;
        let manifest: Manifest = toml::from_str(&content)
            .with_context(|| "Failed to parse Omni.toml data")?;
        Ok(manifest)
    }

    pub fn save_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string(self)
            .with_context(|| "Failed to serialize Manifest")?;
        fs::write(path.as_ref(), content)
            .with_context(|| format!("Failed to write Omni.toml at {:?}", path.as_ref()))?;
        Ok(())
    }
}
