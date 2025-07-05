use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Run {
    pub mode: RunMode,
}

#[derive(Debug, Deserialize)]
pub enum RunMode {
    #[serde(rename = "cli")]
    Cli,
    #[serde(rename = "tui")]
    Tui,
}
