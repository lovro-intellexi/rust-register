use std::{sync::Arc};

use async_trait::async_trait;
use couch_rs::{error::{CouchError, CouchResult}, types::document::DocumentCreatedResult, document::DocumentCollection};
use serde_json::json;

use crate::model::Subject;

use super::Db;

pub struct RegisterAdapter {
    db: Arc<Db>,
}

impl RegisterAdapter {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }
}

#[async_trait]
pub trait DbInteractions {
    async fn get_subject(&self, id: &str) -> Result<Subject, CouchError>;
    async fn create_subject(&self, subject: Subject) -> DocumentCreatedResult;
    async fn get_subject_list(&self) -> CouchResult<DocumentCollection<Subject>>;
}

#[async_trait]
impl DbInteractions for RegisterAdapter {
    async fn get_subject(&self, id: &str) -> Result<Subject, CouchError> {
        self.db.get(id).await
    }

    async fn create_subject(&self, subject: Subject) -> DocumentCreatedResult {
        let mut subject_value = json!(subject);
        self.db.create(&mut subject_value).await
    }

    async fn get_subject_list(&self) -> CouchResult<DocumentCollection<Subject>> {
        self.db.get_all().await
    }
}