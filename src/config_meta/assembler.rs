use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Assembler {
    pub bin: String,
}

impl Default for Assembler {
    fn default() -> Self {
        Self {
            bin: "sb-assembler".to_string(),
        }
    }
}
