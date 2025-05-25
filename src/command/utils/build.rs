use std::fs;
use std::process::Command as StdCommand;

use crate::config::build::InputType;
use crate::config::Config;

pub fn build(config: &Config) -> anyhow::Result<()> {
    // 1. src/main.asm をチェック
    match &config.build.input {
        InputType::Asm => if !fs::exists("./src/main.asm")? {
            return Err(anyhow::anyhow!("src/main.asm not found."));
        }
    }

    // 2. ビルド
    match &config.build.input {
        InputType::Asm => build_asm(config),
    }
}

fn build_asm(_: &Config) -> anyhow::Result<()> {
    // 1. target 準備
    let _ = fs::remove_dir("target/build");
    fs::create_dir_all("target/build")?;

    // 2. アセンブル
    StdCommand::new("sb-assembler")
        .arg("./src/main.asm")
        .arg("./target/build/data.hex")
        .arg("./target/build/inst.hex")
        .status()?;

    Ok(())
}
