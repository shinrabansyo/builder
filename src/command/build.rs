use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use bpaf::Bpaf;

use sb_asm::assemble;

use crate::command::{Command, CliOptions};
use crate::config::build::{InputType, OutputType};
use crate::config::Config;

#[derive(Debug, Clone, Bpaf)]
pub struct Build;

impl From<CliOptions> for Build {
    fn from(cmd: CliOptions) -> Self {
        match cmd {
            CliOptions::Build => Build,
            _ => unreachable!(),
        }
    }
}

impl Command for Build {
    fn run(self) -> anyhow::Result<()> {
        // 1. Package.toml 読み込み
        let config = Config::load("Package.toml")?;

        // 2. 対象ファイル列挙
        let ext_filter = match &config.build.input {
            InputType::Asm => |ext: &OsStr| ext == "asm",
        };
        let src_files = find_files("src/", &ext_filter)?;

        // 3. 実行
        let (data, inst) = match &config.build.input {
            InputType::Asm => {
                let input = src_files
                    .into_iter()
                    .map(|path| fs::read_to_string(path).unwrap())
                    .collect::<String>();
                assemble(&input)?
            }
        };

        // 4. 出力
        let _ = fs::remove_dir("target/build");
        fs::create_dir_all("target/build")?;
        match &config.build.output {
            OutputType::Normal => {
                fs::write("target/build/data.hex", data)?;
                fs::write("target/build/inst.hex", inst)?;
            }
            OutputType::Bank => {
                save_banked::<4>("target/build/data", data.split("\n").into_iter())?;
                save_banked::<6>("target/build/inst", inst.split("\n").into_iter())?;
            }
        }

        Ok(())
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

fn save_banked<'a, const N: usize> (
    prefix: &str,
    elems: impl Iterator<Item = &'a str>,
) -> anyhow::Result<()> {
    let mut outputs = vec![];
    for idx in 0..N {
        let path = format!("{}_{}.hex", prefix, idx);
        outputs.push(fs::File::create(path)?);
    }

    for (idx, elem) in elems.enumerate() {
        outputs[idx % N].write(elem.as_bytes())?;
        outputs[idx % N].write(b"\n")?;
    }

    Ok(())
}

