use std::fs;
use std::process::Command as StdCommand;

use crate::config::build::InputType;
use crate::config::Config;

pub fn build(config: &Config) -> anyhow::Result<()> {
    // 1. src/main.asm or src/main.sb をチェック
    match &config.build.input {
        InputType::Asm => if !fs::exists("./src/main.asm")? {
            return Err(anyhow::anyhow!("src/main.asm not found."));
        }
        InputType::Sblang => if !fs::exists("./src/main.sb")? {
            return Err(anyhow::anyhow!("src/main.sb not found."));
        }
    }

    // 2. ビルド
    match &config.build.input {
        InputType::Asm => build_asm(config),
        InputType::Sblang => build_sblang(config),
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

fn build_sblang(_: &Config) -> anyhow::Result<()> {
    // 1. target 準備
    let _ = fs::remove_dir("target/build");
    fs::create_dir_all("target/build")?;

    // 2. コンパイル
    StdCommand::new("sb-compiler")
        .arg("./src/main.sb")
        .arg("./target/build/main.obj")
        .status()?;

    // 3. リンク
    StdCommand::new("sb-linker")
        .arg("./target/build/main.obj")
        .arg("./target/build/main.asm")
        .status()?;

    // 4. アセンブル
    StdCommand::new("sb-assembler")
        .arg("./target/build/main.asm")
        .arg("./target/build/data.hex")
        .arg("./target/build/inst.hex")
        .status()?;

    Ok(())
}
