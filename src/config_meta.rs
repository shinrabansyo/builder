use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct MetaConfig { }

impl MetaConfig {
    pub fn load_or_default<T: AsRef<Path>>(path: T) -> anyhow::Result<Self> {
        if path.as_ref().exists() {
            let config = fs::read_to_string(path)?;
            let config = toml::from_str::<MetaConfig>(&config)?;
            Ok(config)
        } else {
            Ok(MetaConfig::default())
        }
    }
}
