use bpaf::Bpaf;

use crate::command::Runnable;
use crate::config::Config;

/// Display information about the project
#[derive(Debug, Clone, Bpaf)]
#[bpaf(command("info"))]
pub struct Info;

impl Runnable for Info {
    fn run(self) -> anyhow::Result<()> {
        // 1. Package.toml 読み込み
        let config = Config::load("Package.toml")?;

        // 2. パッケージ情報表示
        println!("Package: {}", config.package.name);
        println!("Version: {}", config.package.version);

        Ok(())
    }
}
