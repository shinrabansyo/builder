use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command as StdCommand;

use crate::command::{Command, CliOptions};

#[derive(Debug, Clone)]
pub struct Oneshot {
    file: PathBuf,
    subcommand: Vec<String>,
}

impl From<CliOptions> for Oneshot {
    fn from(cmd: CliOptions) -> Self {
        match cmd {
            CliOptions::Oneshot { file, subcommand } => Oneshot {
                file,
                subcommand,
            },
            _ => unreachable!(),
        }
    }
}

impl Command for Oneshot {
    fn run(self) -> anyhow::Result<()> {
        let home_dir = env::var("HOME")?;
        let workdir = format!("{}/.shinrabansyo/workdir/builder", home_dir);
        let workdir_src = format!("{}/src", workdir);
        let workdir_src_sb = format!("{}/src/main.sb", workdir);
        let workdir_toml = format!("{}/Package.toml", workdir);

        // 1. 準備
        let _ = fs::remove_dir(&workdir);
        fs::create_dir_all(&workdir_src)?;
        fs::copy(self.file, &workdir_src_sb)?;

        // 2. Packget.toml
        let package_toml = r#"[package]
name = "oneshot"
version = "0.1.0"

[build]
output = ["bin"]  # "bin", "hex-bank", "raw"

[run]
mode = "tui"      # "tui", "cli"

[link]
stack_addr = 0x0000_0100
"#;
        fs::write(&workdir_toml, package_toml)?;

        // 3. コマンド実行
        StdCommand::new("sb-builder")
            .current_dir(&workdir)
            .args(&self.subcommand)
            .status()?;

        Ok(())
    }
}
