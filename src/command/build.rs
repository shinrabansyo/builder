use crate::command::{Command, CliOptions};
use crate::command::utils::build::build;
use crate::command::utils::convert::{convert_bin, convert_hex_bank};
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
        build(&config)?;

        // 3. 出力形式に応じて変換
        for output_opt in &config.build.output {
            match output_opt {
                OutputType::Bank => convert_hex_bank()?,
                OutputType::Bin => convert_bin()?,
            }
        }

        Ok(())
    }
}
