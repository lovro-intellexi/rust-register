use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterDetails {
    mbs: i64,
    oib: i64,
    share_capital: f64
}