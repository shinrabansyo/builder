use std::fs;

use sb_asm::assemble;

use crate::config::build::InputType;
use crate::config::Config;

pub fn build(config: &Config) -> anyhow::Result<(String, String)> {
    // 1. src/main.asm をチェック
    match &config.build.input {
        InputType::Asm => {
            if !fs::exists("./src/main.asm")? {
                return Err(anyhow::anyhow!("src/main.asm not found."));
            }
        }
    }

    // 2. ビルド
    match &config.build.input {
        InputType::Asm => {
            assemble(&fs::read_to_string("./src/main.asm")?)
        }
    }
}
