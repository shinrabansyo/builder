use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command as StdCommand;

use bpaf::Bpaf;

use crate::command::Runnable;
use crate::config_meta::MetaConfig;

const PACKAGE_TOML: &str =
r#"[package]
name = "oneshot"
version = "0.1.0"

[run]
mode = "tui"      # "tui", "cli"
"#;

/// Execute a program without creating a project
#[derive(Debug, Clone, Bpaf)]
#[bpaf(command("oneshot"))]
pub struct Oneshot {
    #[bpaf(long, switch)]
    bin_copy: bool,
    #[bpaf(positional)]
    file: PathBuf,
    #[bpaf(positional("SUB-COMMAND"), many)]
    subcommand: Vec<String>,
}

impl Runnable for Oneshot {
    fn run(self, _: MetaConfig) -> anyhow::Result<()> {
        let home_dir = env::var("HOME")?;
        let workdir = format!("{}/.shinrabansyo/workdir/builder", home_dir);

        // 1. 準備
        let _ = fs::remove_dir_all(&workdir);
        fs::create_dir_all(format!("{}/src", workdir))?;
        fs::copy(self.file, format!("{}/src/main.sb", workdir))?;

        // 2. Packget.toml
        let toml_path = format!("{}/Package.toml", workdir);
        fs::write(toml_path, PACKAGE_TOML)?;

        // 3. コマンド実行
        StdCommand::new("sb-builder")
            .current_dir(&workdir)
            .args(&self.subcommand)
            .status()?;

        // 4. out.bin (コマンド実行結果) をコピー
        if self.bin_copy {
            let bin_path = format!("{}/target/out/bin/out.bin", workdir);
            if fs::exists(&bin_path)? {
                fs::copy(&bin_path, "out.bin")?;
            } else {
                return Err(anyhow::anyhow!("`out.bin` not created."));
            }
        }

        Ok(())
    }
}
