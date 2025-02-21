pub mod build;
pub mod package;

use std::fs;
use std::path::Path;

use serde::Deserialize;

use build::Build;
use package::Package;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub package: Package,
    pub build: Build,
}

impl Config {
    pub fn load<T: AsRef<Path>>(path: T) -> anyhow::Result<Self> {
        let config = fs::read_to_string(path)?;
        let config = toml::from_str::<Config>(&config)?;
        Ok(config)
    }
}
