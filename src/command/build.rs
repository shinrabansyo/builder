use bpaf::Bpaf;

use crate::command::utils::build::build as my_build;
use crate::command::utils::convert::{convert_bin, convert_hex_bank, convert_raw};
use crate::command::Runnable;
use crate::config_meta::MetaConfig;
use crate::config_project::build::OutputType;
use crate::config_project::ProjectConfig;

/// Build the project
#[derive(Debug, Clone, Bpaf)]
#[bpaf(command("build"))]
pub struct Build;

impl Runnable for Build {
    fn run(self, meta_config: MetaConfig) -> anyhow::Result<()> {
        // 1. Package.toml 読み込み
        let prj_config = ProjectConfig::load("Package.toml")?;

        // 2. ビルド
        my_build(&meta_config, &prj_config)?;

        // 3. 出力形式に応じて変換
        for output_opt in &prj_config.build.output {
            match output_opt {
                OutputType::Bin => convert_bin()?,
                OutputType::HexBank => convert_hex_bank()?,
                OutputType::Raw => convert_raw()?,
            }
        }

        Ok(())
    }
}
