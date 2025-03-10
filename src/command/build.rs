use std::fs;
use std::io::Write;

use crate::command::{Command, CliOptions};
use crate::command::utils::build;
use crate::config::build::OutputType;
use crate::config::Config;

#[derive(Debug, Clone)]
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

        // 2. ビルド
        let (data, inst) = build(&config)?;

        // 3. 出力
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
