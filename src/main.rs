mod command;
mod config_meta;
mod config_project;

use std::path::PathBuf;

use bpaf::Bpaf;

use command::{Command, Runnable, command};
use config_meta::MetaConfig;

#[derive(Debug, Bpaf)]
#[bpaf(options, version)]
struct CliOptions {
    /// Path to the meta configuration file (default: '.sb-builder/Config.toml')
    #[bpaf(long, short, fallback("./.sb-builder/Config.toml".into()))]
    meta_config: PathBuf,

    /// Subcommand to execute
    #[bpaf(external)]
    command: Command,
}

fn main() -> anyhow::Result<()> {
    let opts = cli_options()
        .header("A build and run tool for Shinrabansyo Project")
        .usage("Usage: sb_builder_cli [-m=ARG] [COMMAND ...] ")
        .run();
    let meta_config = MetaConfig::load_or_default(&opts.meta_config)?;
    opts.command.run(meta_config)
}
