use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Run {
    pub mode: RunMode,
}

#[derive(Debug, Default, Deserialize)]
pub enum RunMode {
    #[serde(rename = "cli")]
    Cli,
    #[default]
    #[serde(rename = "tui")]
    Tui,
}
