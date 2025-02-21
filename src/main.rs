mod config;

use config::Config;

fn main() -> anyhow::Result<()> {
    println!("{:?}", Config::load("Package.toml")?);
    Ok(())
}
