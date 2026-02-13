use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command as StdCommand;

use crate::config_meta::MetaConfig;
use crate::config_project::ProjectConfig;

const LINK_SCRIPT: &str =
r#"[general]
main = ".main.main"
stack_addr = {%addr%}
"#;

pub fn link(
    meta_config: &MetaConfig,
    prj_config: &ProjectConfig,
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
) -> anyhow::Result<()> {
    // リンカスクリプト準備
    let link_script = LINK_SCRIPT.replace(
        "{%addr%}",
        &prj_config.link.stack_addr.to_string(),
    );
    fs::write("./target/build/link.toml", link_script)?;

    // リンク実行
    let result = StdCommand::new(&meta_config.linker.bin)
        .arg("-c")
        .arg("./target/build/link.toml")
        .arg("-o")
        .arg(output.as_ref())
        .arg(input.as_ref())
        .status();

    match result {
        Ok(_) => Ok(()),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => Err(anyhow::anyhow!(
                "Specified linker binary ('{}') not found.",
                &meta_config.compiler.bin,
            )),
            _ => Err(anyhow::anyhow!(
                "Unknown error occurred: {}",
                err,
            )),
        }
    }
}
