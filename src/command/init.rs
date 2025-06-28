use std::fs;

use crate::command::{Command, CliOptions};

#[derive(Debug, Clone)]
pub struct Init {
    name: String,
}

impl From<CliOptions> for Init {
    fn from(cmd: CliOptions) -> Self {
        match cmd {
            CliOptions::Init { name } => Init { name },
            _ => unreachable!(),
        }
    }
}

impl Command for Init {
    fn run(self) -> anyhow::Result<()> {
        // 1. Package.toml
        let toml_path = "Package.toml";
        let toml_content = format!(
                r#"[package]
name = "{}"
version = "0.1.0"

[build]
input = "sblang"
output = "bank"
"#,
                self.name,
            );
        fs::write(toml_path, toml_content)?;

        // 2. .gitignore
        fs::write(".gitignore", "target*/\n")?;

        // 3. src ディレクトリ
        fs::create_dir("src")?;

        // 4. プログラムのテンプレート
        let sb_path = "src/main.sb";
        let sb_content = r#"fn main() -> i32 {
    return 0;
}
"#;
        fs::write(sb_path, sb_content)?;

        Ok(())
    }
}
