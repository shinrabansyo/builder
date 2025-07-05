use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Link {
    pub stack_addr: u32,
}
