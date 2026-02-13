use bpaf::Bpaf;

use crate::command::Runnable;
use crate::config_meta::MetaConfig;
use crate::config_project::ProjectConfig;

/// Display information about the project
#[derive(Debug, Clone, Bpaf)]
#[bpaf(command("info"))]
pub struct Info;

impl Runnable for Info {
    fn run(self, _: MetaConfig) -> anyhow::Result<()> {
        // 1. Package.toml 読み込み
        let prj_config = ProjectConfig::load("Package.toml")?;

        // 2. パッケージ情報表示
        println!("Package: {}", prj_config.package.name);
        println!("Version: {}", prj_config.package.version);

        Ok(())
    }
}
