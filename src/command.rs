pub mod build;

use bpaf::Bpaf;

use build::BuildOptions;

pub trait Command
where
    Self: From<CliOptions>,
{
    fn run(self) -> anyhow::Result<()>;
}

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub enum CliOptions {
    #[bpaf(command)]
    Build,
}

pub fn parse_args() -> impl Command {
    let opts = cli_options().run();
    match opts {
        CliOptions::Build => BuildOptions::from(opts),
    }
}
