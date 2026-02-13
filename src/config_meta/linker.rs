use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Linker {
    pub bin: String,
}

impl Default for Linker {
    fn default() -> Self {
        Linker {
            bin: "sb-linker".to_string(),
        }
    }
}
