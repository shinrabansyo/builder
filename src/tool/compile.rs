use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command as StdCommand;

use crate::config_meta::MetaConfig;

pub fn compile(
    meta_config: &MetaConfig,
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let result = StdCommand::new(&meta_config.compiler.bin)
        .arg("-o")
        .arg(output.as_ref())
        .arg(input.as_ref())
        .args(listup_lib(&meta_config)?)
        .status();

    match result {
        Ok(_) => Ok(()),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => Err(anyhow::anyhow!(
                "Specified compiler binary ('{}') not found.",
                &meta_config.compiler.bin,
            )),
            _ => Err(anyhow::anyhow!(
                "Unknown error occurred: {}",
                err,
            )),
        }
    }
}

fn listup_lib(meta_config: &MetaConfig) -> anyhow::Result<Vec<PathBuf>> {
    fn __inner(dir: &Path) -> Vec<PathBuf> {
        let mut libs = vec![];
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path: PathBuf = entry.path();
                if path.is_dir() {
                    libs.extend(__inner(&path));
                } else if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("sb") {
                    libs.push(path);
                }
            }
        }
        libs
    }
    Ok(__inner(&Path::new(&meta_config.compiler.lib)))
}
