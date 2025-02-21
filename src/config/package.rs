use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Package {
    name: String,
    version: String,
}
