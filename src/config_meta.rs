pub mod assembler;
pub mod builder;
pub mod compiler;
pub mod linker;

use std::fs;
use std::path::Path;

use serde::Deserialize;

use assembler::Assembler;
use builder::Builder;
use compiler::Compiler;
use linker::Linker;

#[derive(Debug, Default, Deserialize)]
pub struct MetaConfig {
    #[serde(default)]
    pub assembler: Assembler,
    #[serde(default)]
    pub builder: Builder,
    #[serde(default)]
    pub compiler: Compiler,
    #[serde(default)]
    pub linker: Linker,
}

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
