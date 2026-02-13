use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Build {
    pub output: Vec<OutputType>,
}

impl Default for Build {
    fn default() -> Self {
        Self {
            output: vec![OutputType::SelfFmt],
        }
    }
}

#[derive(Debug, Default ,Deserialize)]
pub enum OutputType {
    #[default]
    #[serde(rename = "self")]
    SelfFmt,
    #[serde(rename = "hex-bank")]
    HexBank,
    #[serde(rename = "raw")]
    Raw,
}
