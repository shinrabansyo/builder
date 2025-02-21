use bpaf::Bpaf;

use crate::command::{Command, CliOptions};
use crate::config::Config;

#[derive(Debug, Clone, Bpaf)]
pub struct BuildOptions;

impl From<CliOptions> for BuildOptions {
    fn from(cmd: CliOptions) -> Self {
        match cmd {
            CliOptions::Build => BuildOptions,
            _ => unreachable!(),
        }
    }
}

impl Command for BuildOptions {
    fn run(self) -> anyhow::Result<()> {
        let config = Config::load("Package.toml")?;
        println!("{:?}", config);
        Ok(())
    }
}
