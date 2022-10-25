use std::sync::Arc;

use async_trait::async_trait;
use couch_rs::error::CouchError;

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
    //async fn getSubjectList(&self, limit: Option<String>);
}

#[async_trait]
impl DbInteractions for RegisterAdapter {
    async fn get_subject(&self, id: &str) -> Result<Subject, CouchError> {
        println!("{:?}", id);
        match self.db.get(id).await {
            Ok(sub) => {
                println!("{:?}", sub);

                Ok(sub)
            },
            Err(ex) => {
                println!("{:?}", ex);

                Err(ex)
            }
        }
    }
}