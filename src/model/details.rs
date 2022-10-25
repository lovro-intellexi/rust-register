use couch_rs::{document::TypedCouchDocument, types::document::DocumentId};
use couch_rs::CouchDocument;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, CouchDocument)]
pub struct Details {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: DocumentId,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,
    mbs: u32,
    oib: u32,
    share_capital: String,
}
