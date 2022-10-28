//use std::env;
use couch_rs::{error::CouchError, database::Database};

mod register_adapter;
pub use register_adapter::RegisterAdapter;
pub use register_adapter::DbInteractions;

const DB_HOST: &str = "http://localhost:5984";
const DB_NAME: &str = "sudski_registar";
const DB_USER: &str = "root";
const DB_PASSWORD: &str = "pass";

// TODO: fix const String from env
//const DB_HOST: String = env::var("DB_HOST").expect("DB_HOST...");
//const DB_NAME: String = env::var("DB_NAME").expect("DB_NAME is required");
//const DB_USER: String = env::var("DB_USER").expect("DB_USER is required");
//const DB_PASSWORD: String = env::var("DB_PASSWORD").expect("DB_PASSWORD is required");

pub type Db = Database;

pub async fn init_db() -> Result<Db, CouchError> {
    new_db(DB_HOST, DB_NAME, DB_USER, DB_PASSWORD).await
}

async fn new_db(db_host: &str, db_name: &str, user: &str, password: &str) -> Result<Db, CouchError> {
    let client = couch_rs::Client::new(db_host, user, password)?;
    client.db(db_name).await
}

#[cfg(test)]
#[path = "../_tests/db.rs"]
mod tests;