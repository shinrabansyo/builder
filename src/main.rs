mod command;
mod config;

use std::path::PathBuf;

use bpaf::Bpaf;

use command::build::Build;
use command::info::Info;
use command::init::Init;
use command::new::New;
use command::oneshot::Oneshot;
use command::run::Run;
use command::Command;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub enum CliOptions {
    /// Create a new project
    #[bpaf(command)]
    New {
        #[bpaf(positional, fallback("helloworld".to_string()))]
        name: String,
    },
    /// Initialize a new project in the current directory
    #[bpaf(command)]
    Init {
        #[bpaf(positional, fallback("helloworld".to_string()))]
        name: String,
    },
    /// Display information about the project
    #[bpaf(command)]
    Info,
    /// Build the project
    #[bpaf(command)]
    Build,
    /// Debug the project
    #[bpaf(command)]
    Run,
    /// Oneshot mode, run command on a single file
    #[bpaf(command)]
    Oneshot {
        #[bpaf(long, switch)]
        bin_copy: bool,
        #[bpaf(positional)]
        file: PathBuf,
        #[bpaf(positional("SUB-COMMAND"), many)]
        subcommand: Vec<String>,
    }
}

fn main() -> anyhow::Result<()> {
    let opts = cli_options().run();
    match opts {
        CliOptions::New { .. } => New::from(opts).run(),
        CliOptions::Init { .. } => Init::from(opts).run(),
        CliOptions::Info => Info::from(opts).run(),
        CliOptions::Build => Build::from(opts).run(),
        CliOptions::Run => Run::from(opts).run(),
        CliOptions::Oneshot { .. } => Oneshot::from(opts).run(),
    }
}
