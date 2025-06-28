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
        // 1. ディレクトリ作成
        fs::create_dir_all(&self.name)?;

        // 2. .gitignore
        fs::write(format!("{}/.gitignore", self.name), "target*/\n")?;

        // 3. Package.toml
        let toml_path = format!("{}/Package.toml", self.name);
        let toml_content = format!(
                r#"[package]
name = "{}"
version = "0.1.0"

[build]
input = "sblang"
output = []

[link]
stack_addr = 0x0000_0100
"#,
                self.name,
            );
        fs::write(toml_path, toml_content)?;

        // 4. src ディレクトリ
        fs::create_dir(format!("{}/src", self.name))?;

        // 5. プログラムのテンプレート
        let sb_path = format!("{}/src/main.sb", self.name);
        let sb_content = r#"fn main() -> i32 {
    return 0;
}
"#;
        fs::write(sb_path, sb_content)?;

        Ok(())
    }
}
