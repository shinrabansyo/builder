use bpaf::Bpaf;

use crate::command::{Command, CliOptions};
use crate::config::Config;

#[derive(Debug, Clone, Bpaf)]
pub struct Info;

impl From<CliOptions> for Info {
    fn from(cmd: CliOptions) -> Self {
        match cmd {
            CliOptions::Info => Info,
            _ => unreachable!(),
        }
    }
}

impl Command for Info {
    fn run(self) -> anyhow::Result<()> {
        // 1. Package.toml 読み込み
        let config = Config::load("Package.toml")?;

        // 2. パッケージ情報表示
        println!("Package: {}", config.package.name);
        println!("Version: {}", config.package.version);

        Ok(())
    }
}
