use std::fs;

use crate::command::{Command, CliOptions};

#[derive(Debug, Clone)]
pub struct New {
    name: String,
}

impl From<CliOptions> for New {
    fn from(cmd: CliOptions) -> Self {
        match cmd {
            CliOptions::New { name } => New { name },
            _ => unreachable!(),
        }
    }
}

impl Command for New {
    fn run(self) -> anyhow::Result<()> {
        // 1. Package.toml
        fs::write(
            "Package.toml",
            format!(
                r#"[package]
name = "{}"
version = "0.1.0"

[build]
input = "asm"
output = "bank"
"#,
                self.name,
            ),
        )?;

        // 2. src ディレクトリ
        fs::create_dir("src")?;
        fs::write("src/main.asm", "\n===\n\n")?;

        Ok(())
    }
}
