pub mod build;
pub mod run;
pub mod info;
pub mod init;
pub mod new;
mod utils;

use crate::CliOptions;

pub trait Command
where
    Self: From<CliOptions>,
{
    fn run(self) -> anyhow::Result<()>;
}
