use std::fs;

use crate::config_meta::MetaConfig;
use crate::config_project::ProjectConfig;
use crate::tool::{assemble, compile, link};

pub fn build(
    meta_config: &MetaConfig,
    prj_config: &ProjectConfig,
) -> anyhow::Result<()> {
    // 準備
    if !fs::exists("./src/main.sb")? {
        return Err(anyhow::anyhow!("src/main.sb not found."));
    }
    let _ = fs::remove_dir("target/build");
    let _ = fs::remove_dir("target/out");
    fs::create_dir_all("target/build")?;
    fs::create_dir_all("target/out/hex")?;

    // ビルド実行 (.sb -> .obj -> .asm -> .hex)
    compile(
        meta_config,
        "./src/main.sb",
        "./target/build/main.obj",
    )?;
    link(
        meta_config,
        prj_config,
        "./target/build/main.obj",
        "./target/build/main.asm",
    )?;
    assemble(
        meta_config,
        "./target/build/main.asm",
        "./target/out/hex/inst.hex",
        "./target/out/hex/data.hex",
    )?;

    Ok(())
}
