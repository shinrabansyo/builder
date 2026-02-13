use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Builder {
    pub bin: String,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            bin: "sb-builder".to_string(),
        }
    }
}
