use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Link {
    pub stack_addr: u32,
}

impl Default for Link {
    fn default() -> Self {
        Link { stack_addr: 0x0000_1000 }
    }
}
