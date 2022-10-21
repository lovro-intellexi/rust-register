use couch_rs::CouchDocument;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, CouchDocument)]
pub struct Details {
    mbs: u32,
    oib: u32,
    share_capital: String,
}
