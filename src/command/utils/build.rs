use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use sb_asm::assemble;

use crate::config::build::InputType;
use crate::config::Config;

pub fn build(config: &Config) -> anyhow::Result<(String, String)> {
    // 1. 対象ファイル列挙
    let ext_filter = match &config.build.input {
        InputType::Asm => |ext: &OsStr| ext == "asm",
    };
    let src_files = find_files("src/", &ext_filter)?;

    // 2. ビルド
    match &config.build.input {
        InputType::Asm => {
            let input = src_files
                .into_iter()
                .map(|path| fs::read_to_string(path).unwrap())
                .collect::<String>();
            assemble(&input)
        }
    }
}

fn find_files<T, F>(path: T, ext_filter: &F) -> anyhow::Result<Vec<PathBuf>>
where
    T: AsRef<Path>,
    F: Fn(&OsStr) -> bool,
{
    let mut files = vec![];
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            find_files(&path, ext_filter)?;
        } else if ext_filter(path.extension().unwrap_or_default()) {
            files.push(path);
        }
    }
    Ok(files)
}
