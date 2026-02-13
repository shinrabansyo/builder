use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Build {
    pub output: Vec<OutputType>,
}

#[derive(Debug, Default ,Deserialize)]
pub enum OutputType {
    #[default]
    #[serde(rename = "bin")]
    Bin,
    #[serde(rename = "hex-bank")]
    HexBank,
    #[serde(rename = "raw")]
    Raw,
}
