use std::fs;

use bpaf::Bpaf;

use crate::command::Runnable;

/// Initialize a new project in the current directory
#[derive(Debug, Clone, Bpaf)]
#[bpaf(command("init"))]
pub struct Init {
    #[bpaf(positional, fallback("helloworld".to_string()))]
    name: String,
}

impl Runnable for Init {
    fn run(self) -> anyhow::Result<()> {
        // 1. Package.toml
        let toml_path = "Package.toml";
        let toml_content = format!(
                r#"[package]
name = "{}"
version = "0.1.0"

[build]
output = ["bin"]  # "bin", "hex-bank", "raw"

[run]
mode = "tui"      # "cli", "tui"

[link]
stack_addr = 0x0000_0100
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
