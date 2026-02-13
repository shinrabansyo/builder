use std::fs;

use bpaf::Bpaf;

use crate::command::Runnable;

const GITIGNORE: &str =
r#"target*/
"#;

const PACKAGE_TOML: &str =
r#"[package]
name = "{%name%}"
version = "0.1.0"

[build]
output = ["bin"]  # "bin", "hex-bank", "raw"

[run]
mode = "tui"      # "tui", "cli"

[link]
stack_addr = 0x0000_0100
"#;

const MAIN_SB: &str =
r#"fn main() -> i32 {
    return 0;
}
"#;

/// Create a new project
#[derive(Debug, Clone, Bpaf)]
#[bpaf(command("new"))]
pub struct New {
    #[bpaf(positional, fallback("helloworld".to_string()))]
    name: String,
}

impl Runnable for New {
    fn run(self) -> anyhow::Result<()> {
        // 1. ディレクトリ作成
        fs::create_dir_all(&self.name)?;

        // 2. .gitignore
        fs::write(format!("{}/.gitignore", self.name), GITIGNORE)?;

        // 3. Package.toml
        let toml_path = format!("{}/Package.toml", self.name);
        let toml_content = PACKAGE_TOML.replace("{%name%}", &self.name);
        fs::write(toml_path, toml_content)?;

        // 4. src ディレクトリ
        fs::create_dir(format!("{}/src", self.name))?;

        // 5. プログラムのテンプレート
        let sb_path = format!("{}/src/main.sb", self.name);
        fs::write(sb_path, MAIN_SB)?;

        Ok(())
    }
}
