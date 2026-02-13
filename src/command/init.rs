use std::fs;

use bpaf::Bpaf;

use crate::command::Runnable;
use crate::config_meta::MetaConfig;

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
mode = "tui"      # "cli", "tui"

[link]
stack_addr = 0x0000_0100
"#;

const MAIN_SB: &str =
r#"fn main() -> i32 {
    return 0;
}
"#;

/// Initialize a new project in the current directory
#[derive(Debug, Clone, Bpaf)]
#[bpaf(command("init"))]
pub struct Init {
    #[bpaf(positional, fallback("helloworld".to_string()))]
    name: String,
}

impl Runnable for Init {
    fn run(self, _: MetaConfig) -> anyhow::Result<()> {
        // 1. .gitignore
        fs::write(".gitignore", GITIGNORE)?;

        // 2. Package.toml
        fs::write(
            "Package.toml",
            PACKAGE_TOML.replace("{%name%}", &self.name),
        )?;

        // 3. src ディレクトリ
        fs::create_dir("src")?;

        // 4. プログラムのテンプレート
        fs::write("src/main.sb", MAIN_SB)?;

        Ok(())
    }
}
