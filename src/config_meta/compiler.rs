use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Compiler {
    pub bin: String,
}

impl Default for Compiler {
    fn default() -> Self {
        Compiler {
            bin: "sb-compiler".to_string(),
        }
    }
}
