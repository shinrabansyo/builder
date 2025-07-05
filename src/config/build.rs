use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Build {
    pub output: Vec<OutputType>,
}

#[derive(Debug, Deserialize)]
pub enum OutputType {
    #[serde(rename = "bank")]
    Bank,
    #[serde(rename = "raw")]
    Raw,
}
