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
    let _ = fs::remove_dir("target/out");
    fs::create_dir_all("target/out")?;

    // 2. アセンブル
    let status = StdCommand::new("sb-assembler")
        .arg("./src/main.asm")
        .arg("./target/out/data.hex")
        .arg("./target/out/inst.hex")
        .status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Assemble process failed."));
    }

    Ok(())
}

fn build_sblang(config: &Config) -> anyhow::Result<()> {
    // 1. target 準備
    let _ = fs::remove_dir("target/build");
    let _ = fs::remove_dir("target/out");
    fs::create_dir_all("target/build")?;
    fs::create_dir_all("target/out")?;

    // 2. コンパイル
    let status = StdCommand::new("sb-compiler")
        .arg("./src/main.sb")
        .arg("./target/build/main.obj")
        .status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Compile failed."));
    }

    // 3. リンク
    let script = format!(r#"[general]
main = ".main.main"
stack_addr = {}
"#,
        config.link.stack_addr,
    );
    fs::write("./target/build/link.toml", script)?;

    let status = StdCommand::new("sb-linker")
        .arg("./target/build/link.toml")
        .arg("./target/build/main.obj")
        .arg("./target/build/main.asm")
        .status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Link failed."));
    }

    // 4. アセンブル
    let status = StdCommand::new("sb-assembler")
        .arg("./target/build/main.asm")
        .arg("./target/out/data.hex")
        .arg("./target/out/inst.hex")
        .status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Assemble failed."));
    }

    Ok(())
}
