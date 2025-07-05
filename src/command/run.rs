use std::process::Command as StdCommand;

use crate::command::{Command, CliOptions};
use crate::command::utils::build::build;
use crate::config::run::RunMode;
use crate::config::Config;

#[derive(Debug, Clone)]
pub struct Run;

impl From<CliOptions> for Run {
    fn from(cmd: CliOptions) -> Self {
        match cmd {
            CliOptions::Run => Run,
            _ => unreachable!(),
        }
    }
}

impl Command for Run {
    fn run(self) -> anyhow::Result<()> {
        // 1. Package.toml 読み込み
        let config = Config::load("Package.toml")?;

        // 2. ビルド
        build(&config)?;

        // 3. エミュレータ起動
        let mut cmd = match config.run.mode {
            RunMode::Cli => StdCommand::new("sb-emulator-cli"),
            RunMode::Tui => StdCommand::new("sb-emulator-tui"),
        };
        cmd.arg("--format")
            .arg("bytechar")
            .arg("--data")
            .arg("./target/out/hex/data.hex")
            .arg("--inst")
            .arg("./target/out/hex/inst.hex")
            .status()?;

        Ok(())
    }
}
