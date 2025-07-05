pub mod build;
pub mod info;
pub mod init;
pub mod new;
pub mod run;
pub mod oneshot;
mod utils;

use crate::CliOptions;

pub trait Command
where
    Self: From<CliOptions>,
{
    fn run(self) -> anyhow::Result<()>;
}
