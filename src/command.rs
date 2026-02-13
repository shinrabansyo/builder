mod build;
mod info;
mod init;
mod new;
mod run;
mod oneshot;

use bpaf::Bpaf;

use crate::config_meta::MetaConfig;

use build::{Build, build};
use info::{Info, info};
use init::{Init, init};
use new::{New, new};
use oneshot::{Oneshot, oneshot};
use run::{Run, run};

pub trait Runnable {
    fn run(self, meta_config: MetaConfig) -> anyhow::Result<()>;
}

#[derive(Debug, Bpaf)]
pub struct Command {
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

impl Runnable for Command {
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
