use couch_rs::{document::TypedCouchDocument, types::document::DocumentId};
use couch_rs::CouchDocument;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, CouchDocument, Debug)]
pub struct Subject {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: DocumentId,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,
    //mbs: i64,
    //status: i32,
    //sud_id_nadlezan: i64,
    //sud_id_sluzba: i64,
    pub oib: String,
    //ino_podruznica: i32,
}