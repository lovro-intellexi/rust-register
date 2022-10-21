use couch_rs::document::TypedCouchDocument;
use couch_rs::error::CouchResult;
use couch_rs::types::document::DocumentId;
use couch_rs::CouchDocument;
use serde::{Deserialize, Serialize};
//use serde_json::to_value;
//use std::env;

//TODO: fix String / &str error
//const DB_HOST: String = env::var("DB_HOST").expect("DB_HOST not found");
const DB_HOST: &str = "http://localhost:5984";
const TEST_DB: &str = "sudski_registar";

#[derive(Serialize, Deserialize, CouchDocument, Debug)]
pub struct Subject {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: DocumentId,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,
    #[serde(rename = "oib")]
    pub oib: Option<String>,
    #[serde(rename = "ime")]
    pub name: String,
}

#[tokio::main]
async fn main() -> CouchResult<()> {
    let client = couch_rs::Client::new(&DB_HOST, "root", "pass")?;
    let db = client.db(TEST_DB).await?;

    // Insert subject into DB
    /*let subject_to_save = Subject {
        _id: "123".to_string(),
        _rev: "".to_string(),
        oib: None,
        name: "Test".to_string(),
    };
    let mut value = to_value(subject_to_save)?;
    db.create(&mut value).await?;

    // Change subject data
    let mut subject: Subject = db.get("123").await?;
    subject.oib = Some("123456789".to_string()); */

    // Get subject from DB
    let _subject: Subject = db.get("123").await?;

    println!("{:?}", _subject);

    Ok(())
}
