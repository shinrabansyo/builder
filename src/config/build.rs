use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Build {
    pub output: Vec<OutputType>,
}

#[derive(Debug, Deserialize)]
pub enum OutputType {
    #[serde(rename = "bin")]
    Bin,
    #[serde(rename = "hex-bank")]
    HexBank,
    #[serde(rename = "raw")]
    Raw,
}
