use std::env;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Compiler {
    pub bin: String,
    pub lib_dir: PathBuf,
}

impl Default for Compiler {
    fn default() -> Self {
        let home_dir = env::var("HOME").unwrap();
        let lib_dir = format!("{}/.shinrabansyo/repos/compiler/library", home_dir);

        Compiler {
            bin: "sb-compiler".to_string(),
            lib_dir: lib_dir.into(),
        }
    }
}
