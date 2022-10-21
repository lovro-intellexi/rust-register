use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Subject {
    oib: u32,
    name: String,
}

impl Subject {
    pub fn new() -> Subject {}
}
