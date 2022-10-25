use std::sync::Arc;

use async_trait::async_trait;
use couch_rs::error::CouchError;

use crate::{db::{RegisterAdapter, DbInteractions}, model::Subject};

#[derive(Clone)]
pub struct Handler {
    pub register_adapter: Arc<RegisterAdapter>,
}

impl Handler {
    pub fn new(register_adapter: Arc<RegisterAdapter>) -> Self {
        Self{
            register_adapter
        }
    }
}

#[async_trait]
pub trait HandlerInt {
    async fn get_subject(&self, id: &str) -> Result<Subject, CouchError>;
}

#[async_trait]
impl HandlerInt for Handler {
    async fn get_subject(&self, id: &str) -> Result<Subject, CouchError> {
        let subject: Subject = self.register_adapter.get_subject(id).await?;
        Ok(subject)
    }
}