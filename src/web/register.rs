use std::collections::HashMap;
use std::{sync::Arc};

use couch_rs::document::DocumentCollection;
use couch_rs::error::{CouchError, CouchResult};
use couch_rs::types::document::{DocumentCreatedResult};
use reqwest::StatusCode;
use serde::Serialize;
use warp::{Filter};

use crate::handler;
use crate::handler::handler::{Handler, HandlerInt};
use crate::model::{Subject, RegisterSubject, RegisterDetails, Details};
use crate::util::{with_handler, handle_subjects_from_register, check_db_for_new_subjects, handle_get_subject_details};

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
    let subject = warp::path!("subject")
        .and(warp::get())
        .and(with_handler(handler.clone()))
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

    // CreateSubject
    let create_subject = warp::path!("subject")
        .and(warp::post())
        .and(with_handler(handler.clone()))
        .then(|handler| async move {
            let result = handle_create_subject(handler).await;
            match result {
                Ok(response) => {
                    let json = warp::reply::json(&response);
                    Box::new(warp::reply::with_status(json, StatusCode::OK))
                }
                Err(err) => {
                    let json = warp::reply::json(&Failure::new(StatusCode::INTERNAL_SERVER_ERROR, format!("Requset failed: {:?}", err)));
                    Box::new(warp::reply::with_status(json, StatusCode::INTERNAL_SERVER_ERROR))
                }
            }
        });

    // GetSubjectList
    let subject_list = warp::path!("getSubject")
        .and(warp::get())
        .and(with_handler(handler.clone()))
        .then(|handler| async move {
            let result = handle_get_subject_list(handler).await;
            match result {
                Ok(response) => {
                    let json = warp::reply::json(&response.get_data());
                    Box::new(warp::reply::with_status(json, StatusCode::OK))
                }
                Err(err) => {
                    let json = warp::reply::json(&Failure::new(StatusCode::INTERNAL_SERVER_ERROR, format!("Requset failed: {:?}", err)));
                    Box::new(warp::reply::with_status(json, StatusCode::INTERNAL_SERVER_ERROR))
                }
            }
        });

    let get_subjects = warp::path!("getSubjects")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .map(|param: HashMap<String, String>| match param.get("limit") {
            Some(limit) => {println!("limit = {}", limit); limit.clone()},
            //TODO handle missing limit case
            None => "0".to_string(),
        })
        .and(with_handler(handler.clone()))
        .then(|limit: String, handler| async move {
            let subjects_from_register: Vec<RegisterSubject> = handle_subjects_from_register(limit.clone()).await;
            check_db_for_new_subjects(subjects_from_register).await;
            //TODO add handle function
            let result = handle_get_subject_list(handler).await;
            match result {
                Ok(response) => {
                    let json = warp::reply::json(&response.get_data());
                    Box::new(warp::reply::with_status(json, StatusCode::OK))
                }
                Err(err) => {
                    let json = warp::reply::json(&Failure::new(StatusCode::INTERNAL_SERVER_ERROR, format!("Requset failed: {:?}", err)));
                    Box::new(warp::reply::with_status(json, StatusCode::INTERNAL_SERVER_ERROR))
                }
            }
        });

    let get_subject_details = warp::path!("getSubjectDetails")
        .and(warp::get())
        .and(warp::query::<HashMap<String, i64>>())
        .map(|param: HashMap<String, i64>| match param.get("oib") {
            Some(oib) => {println!("oib = {}", oib); oib.clone()},
            //TODO handle no oib
            None => 0,
        })
        .and(with_handler(handler))
        .then(|oib: i64, handler: Arc<Handler>| async move {
            let details_from_db: Result<DocumentCollection<Details>, CouchError> = handle_get_details_from_db(handler.clone(), oib).await;
            println!("{:?}", details_from_db.unwrap().rows.into_iter().nth(0));
            let subject_from_register: RegisterDetails = handle_get_subject_details(oib.to_string().clone()).await;
            println!("{:?}", subject_from_register);
            //TODO add handle function
            let result = handle_get_subject(handler).await;
            match result {
                Ok(response) => {
                    let json = warp::reply::json(&response);
                    Box::new(warp::reply::with_status(json, StatusCode::OK))
                }
                Err(err) => {
                    let json = warp::reply::json(&Failure::new(StatusCode::INTERNAL_SERVER_ERROR, format!("Requset failed: {:?}", err)));
                    Box::new(warp::reply::with_status(json, StatusCode::INTERNAL_SERVER_ERROR))
                }
            }
        });

    subject.or(subject_list).or(create_subject).or(get_subjects).or(get_subject_details)
}

//TODO fix result type (DocumentCollection, CreateResult...)
/*fn get_response(result: CouchResult<ResponseResult>) -> Box<warp::reply::WithStatus<Json>> {
    match result {
        Ok(response) => {
            let json = warp::reply::json(&response);
            Box::new(warp::reply::with_status(json, StatusCode::OK))
        }
        Err(err) => {
            let json = warp::reply::json(&Failure::new(StatusCode::INTERNAL_SERVER_ERROR, format!("Requset failed: {:?}", err)));
            Box::new(warp::reply::with_status(json, StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}*/

async fn handle_get_subject(handler: Arc<Handler>) -> Result<Subject, CouchError> {
    handler.get_subject("123").await
}

async fn handle_create_subject(handler: Arc<Handler>) -> DocumentCreatedResult {
    let subject = Subject{
        _id: "".to_string(),
        _rev: "".to_string(),
        oib: 123456789,
    };
    handler.create_subject(subject).await
}

async fn handle_get_subject_list(handler: Arc<Handler>) -> CouchResult<DocumentCollection<Subject>> {
    handler.get_subject_list().await
}

async fn handle_get_details_from_db(handler: Arc<Handler>, oib: i64) -> Result<DocumentCollection<Details>, CouchError> {
    handler.get_details(oib).await
}