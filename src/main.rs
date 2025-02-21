mod command;
mod config;

use bpaf::Bpaf;

use command::build::Build;
use command::info::Info;
use command::debug::Debug;
use command::Command;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub enum CliOptions {
    #[bpaf(command)]
    Info,
    #[bpaf(command)]
    Build,
    #[bpaf(command)]
    Debug,
}

fn main() -> anyhow::Result<()> {
    let opts = cli_options().run();
    match opts {
        CliOptions::Info => Info::from(opts).run(),
        CliOptions::Build => Build::from(opts).run(),
        CliOptions::Debug => Debug::from(opts).run(),
    }
}
