use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Build {
    pub input: InputType,
    pub output: OutputType,
}

#[derive(Debug, Deserialize)]
pub enum InputType {
    #[serde(rename = "asm")]
    Asm,
    #[serde(rename = "sblang")]
    Sblang,
}

#[derive(Debug, Deserialize)]
pub enum OutputType {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "bank")]
    Bank,
}
