pub mod build;
pub mod link;
pub mod package;
pub mod run;

use std::fs;
use std::path::Path;

use serde::Deserialize;

use build::Build;
use link::Link;
use package::Package;
use run::Run;

#[derive(Debug, Deserialize)]
pub struct ProjectConfig {
    pub package: Package,
    #[serde(default)]
    pub build: Build,
    #[serde(default)]
    pub run: Run,
    #[serde(default)]
    pub link: Link,
}

impl ProjectConfig {
    pub fn load<T: AsRef<Path>>(path: T) -> anyhow::Result<Self> {
        let config = fs::read_to_string(path)?;
        let config = toml::from_str::<ProjectConfig>(&config)?;
        Ok(config)
    }
}
