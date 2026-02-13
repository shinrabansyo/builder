use std::process::Command as StdCommand;

use bpaf::Bpaf;

use crate::command::utils::build::build;
use crate::command::Runnable;
use crate::config_meta::MetaConfig;
use crate::config_project::run::RunMode;
use crate::config_project::ProjectConfig;

/// Debug the project
#[derive(Debug, Clone, Bpaf)]
#[bpaf(command("run"))]
pub struct Run;

impl Runnable for Run {
    fn run(self, _: MetaConfig) -> anyhow::Result<()> {
        // 1. Package.toml 読み込み
        let prj_config = ProjectConfig::load("Package.toml")?;

        // 2. ビルド
        build(&prj_config)?;

        // 3. エミュレータ起動
        let mut cmd = match prj_config.run.mode {
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
