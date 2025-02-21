mod command;
mod config;

use bpaf::Bpaf;

use command::build::Build;
use command::debug::Debug;
use command::info::Info;
use command::init::Init;
use command::new::New;
use command::Command;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub enum CliOptions {
    /// Create a new project in an existsing directory
    #[bpaf(command)]
    New {
        #[bpaf(short, long, fallback("helloworld".to_string()))]
        name: String,
    },
    /// Initialize a new project in the current directory
    #[bpaf(command)]
    Init {
        #[bpaf(short, long, fallback("helloworld".to_string()))]
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
    Debug,
}

fn main() -> anyhow::Result<()> {
    let opts = cli_options().run();
    match opts {
        CliOptions::New { .. } => New::from(opts).run(),
        CliOptions::Init { .. } => Init::from(opts).run(),
        CliOptions::Info => Info::from(opts).run(),
        CliOptions::Build => Build::from(opts).run(),
        CliOptions::Debug => Debug::from(opts).run(),
    }
}
