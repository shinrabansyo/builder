pub mod build;
pub mod info;
pub mod init;
pub mod new;
pub mod run;
pub mod oneshot;
mod utils;

pub trait Runnable {
    fn run(self) -> anyhow::Result<()>;
}
