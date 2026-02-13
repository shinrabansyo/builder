mod command;
mod config_project;

use bpaf::Bpaf;

use command::build::{Build, build};
use command::info::{Info, info};
use command::init::{Init, init};
use command::new::{New, new};
use command::oneshot::{Oneshot, oneshot};
use command::run::{Run, run};
use command::Runnable;

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
    fn run(self) -> anyhow::Result<()> {
        self.new.map(|cmd| cmd.run());
        self.init.map(|cmd| cmd.run());
        self.info.map(|cmd| cmd.run());
        self.build.map(|cmd| cmd.run());
        self.run.map(|cmd| cmd.run());
        self.oneshot.map(|cmd| cmd.run());
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    cli_options().run().run()
}
