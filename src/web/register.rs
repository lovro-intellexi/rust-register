use std::{sync::Arc};

use couch_rs::error::CouchError;
use reqwest::StatusCode;
use serde::Serialize;
use warp::{Filter};

use crate::handler::handler::{Handler, HandlerInt};
use crate::model::Subject;
use crate::util::with_handler;

#[derive(Serialize)]
pub struct Failure {
    pub code: u16,
    pub message: String,
    pub status: String,
}

impl Failure {
    pub fn new(code: StatusCode, message: String) -> Self {
        Failure {
            code: code.as_u16(),
            message,
            status: code.to_string(),
        }
    }
}

pub fn register_handler(handler: Arc<Handler>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // GetSubject
    let subject = warp::path!("getSubject")
        .and(warp::get())
        .and(with_handler(handler))
        .then(|handler| async move {
            let result = handle_get_subject(handler).await;
            match result {
                Ok(response) => {
                    let json = warp::reply::json(&response);
                    Box::new(warp::reply::with_status(json, StatusCode::OK))
                }
                Err(_err) => {
                    let json = warp::reply::json(&Failure::new(StatusCode::INTERNAL_SERVER_ERROR, "Ne ide".to_string()));
                    Box::new(warp::reply::with_status(json, StatusCode::INTERNAL_SERVER_ERROR))
                }
            }
        });

    subject
}

async fn handle_get_subject(handler: Arc<Handler>) -> Result<Subject, CouchError> {
    handler.get_subject("123").await
}