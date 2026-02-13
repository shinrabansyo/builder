use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as StdCommand;

use crate::config_project::ProjectConfig;

pub fn build(prj_config: &ProjectConfig) -> anyhow::Result<()> {
    // 1. 準備
    if !fs::exists("./src/main.sb")? {
        return Err(anyhow::anyhow!("src/main.sb not found."));
    }
    let _ = fs::remove_dir("target/build");
    let _ = fs::remove_dir("target/out");
    fs::create_dir_all("target/build")?;
    fs::create_dir_all("target/out/hex")?;

    // 2. コンパイル
    let status = StdCommand::new("sb-compiler")
        .arg("-o")
        .arg("./target/build/main.obj")
        .arg("./src/main.sb")
        .args(listup_lib()?)
        .status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Compile failed."));
    }

    // 3. リンク
    let script = format!(r#"[general]
main = ".main.main"
stack_addr = {}
"#,
        prj_config.link.stack_addr,
    );
    fs::write("./target/build/link.toml", script)?;

    let status = StdCommand::new("sb-linker")
        .arg("-c")
        .arg("./target/build/link.toml")
        .arg("-o")
        .arg("./target/build/main.asm")
        .arg("./target/build/main.obj")
        .status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Link failed."));
    }

    // 4. アセンブル
    let status = StdCommand::new("sb-assembler")
        .arg("./target/build/main.asm")
        .arg("./target/out/hex/data.hex")
        .arg("./target/out/hex/inst.hex")
        .status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Assemble failed."));
    }

    Ok(())
}

fn listup_lib() -> anyhow::Result<Vec<PathBuf>> {
    fn __inner(dir: &Path) -> Vec<PathBuf> {
        let mut libs = vec![];
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path: PathBuf = entry.path();
                if path.is_dir() {
                    libs.extend(__inner(&path));
                } else if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("sb") {
                    libs.push(path);
                }
            }
        }
        libs
    }

    let home_dir = env::var("HOME")?;
    let lib_dir = format!("{}/.shinrabansyo/repos/compiler/library", home_dir);
    Ok(__inner(&Path::new(&lib_dir)))
}
