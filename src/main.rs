mod command;
mod config;

use command::{parse_args, Command};

fn main() -> anyhow::Result<()> {
    let command = parse_args();
    command.run()
}
