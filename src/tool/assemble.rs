use std::io::ErrorKind;
use std::path::Path;
use std::process::Command as StdCommand;

use crate::config_meta::MetaConfig;

pub fn assemble(
    meta_config: &MetaConfig,
    input: impl AsRef<Path>,
    output_inst: impl AsRef<Path>,
    output_data: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let result = StdCommand::new(&meta_config.assembler.bin)
        .arg(input.as_ref())
        .arg(output_data.as_ref())
        .arg(output_inst.as_ref())
        .status();

    match result {
        Ok(_) => Ok(()),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => Err(anyhow::anyhow!(
                "Specified assembler binary ('{}') not found.",
                &meta_config.compiler.bin,
            )),
            _ => Err(anyhow::anyhow!(
                "Unknown error occurred: {}",
                err,
            )),
        }
    }
}
