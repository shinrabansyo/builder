use bpaf::Bpaf;

use crate::command::{Command, CliOptions};
use crate::config::Config;

#[derive(Debug, Clone, Bpaf)]
pub struct InfoOptions;

impl From<CliOptions> for InfoOptions {
    fn from(cmd: CliOptions) -> Self {
        match cmd {
            CliOptions::Info => InfoOptions,
            _ => unreachable!(),
        }
    }
}

impl Command for InfoOptions {
    fn run(self) -> anyhow::Result<()> {
        // 1. Package.toml 読み込み
        let config = Config::load("Package.toml")?;

        // 2. パッケージ情報表示
        println!("Package: {}", config.package.name);
        println!("Version: {}", config.package.version);

        Ok(())
    }
}
