mod command;
mod config;

use bpaf::Bpaf;

use command::build::BuildOptions;
use command::info::InfoOptions;
use command::Command;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub enum CliOptions {
    #[bpaf(command)]
    Build,
    #[bpaf(command)]
    Info,
}

fn main() -> anyhow::Result<()> {
    let opts = cli_options().run();
    match opts {
        CliOptions::Build => BuildOptions::from(opts).run(),
        CliOptions::Info => InfoOptions::from(opts).run(),
    }
}
