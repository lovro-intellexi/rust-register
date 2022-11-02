use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterDetails {
    pub mbs: i64,
    pub oib: i64,
}