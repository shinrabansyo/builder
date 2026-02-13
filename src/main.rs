mod command;
mod config_meta;
mod config_project;

use bpaf::Bpaf;

use command::build::{Build, build};
use command::info::{Info, info};
use command::init::{Init, init};
use command::new::{New, new};
use command::oneshot::{Oneshot, oneshot};
use command::run::{Run, run};
use command::Runnable;
use config_meta::MetaConfig;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
struct CliOptions {
    // プロジェクト管理系
    #[bpaf(external, optional)]
    new: Option<New>,
    #[bpaf(external, optional)]
    init: Option<Init>,
    #[bpaf(external, optional)]
    info: Option<Info>,

    // ビルド・実行系
    #[bpaf(external, optional)]
    build: Option<Build>,
    #[bpaf(external, optional)]
    run: Option<Run>,

    // その他
    #[bpaf(external, optional)]
    oneshot: Option<Oneshot>,
}

impl Runnable for CliOptions {
    fn run(self, meta_config: MetaConfig) -> anyhow::Result<()> {
        macro_rules! run_cmd {
            ($cmd:expr) => {
                if let Some(cmd) = $cmd {
                    return cmd.run(meta_config);
                }
            };
        }

        run_cmd!(self.info);
        run_cmd!(self.new);
        run_cmd!(self.init);
        run_cmd!(self.build);
        run_cmd!(self.run);
        run_cmd!(self.oneshot);

        Err(anyhow::anyhow!("No command specified. Use --help to see available commands."))
    }
}

fn main() -> anyhow::Result<()> {
    cli_options()
        .run()
        .run(MetaConfig::load_or_default("./.sb-builder/Config.toml")?)
}
