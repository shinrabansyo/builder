use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Build {
    input: InputType,
    output: OutputType,
}

#[derive(Debug, Deserialize)]
pub enum InputType {
    #[serde(rename = "asm")]
    Asm,
}

#[derive(Debug, Deserialize)]
pub enum OutputType {
    #[serde(rename = "normal")]
    Normal,
}
