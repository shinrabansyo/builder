pub mod build;
pub mod info;
pub mod init;
pub mod new;
pub mod run;
pub mod oneshot;
mod utils;

use crate::config_meta::MetaConfig;

pub trait Runnable {
    fn run(self, meta_config: MetaConfig) -> anyhow::Result<()>;
}
