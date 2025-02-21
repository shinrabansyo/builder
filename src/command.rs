pub mod build;
pub mod info;
pub mod debug;
mod utils;

use crate::CliOptions;

pub trait Command
where
    Self: From<CliOptions>,
{
    fn run(self) -> anyhow::Result<()>;
}
