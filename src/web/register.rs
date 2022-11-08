use std::collections::HashMap;
use std::sync::Arc;

use couch_rs::document::DocumentCollection;
use couch_rs::error::{CouchError, CouchResult};
use couch_rs::types::document::DocumentCreatedResult;
use reqwest::StatusCode;
use serde::Serialize;
use warp::Filter;

use crate::handler::{Handler, HandlerInt};
use crate::model::{Subject, RegisterSubject, RegisterDetails, Details, Error};
use crate::util::{with_handler, get_subjects_from_register, get_subject_details, map_details, map_subjects_from_register};

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

    let get_subjects = warp::path!("getSubjects")
        .and(warp::get())
        .and(warp::query::<HashMap<String, u64>>())
        .map(|param: HashMap<String, u64>| match param.get("limit") {
            Some(limit) =>  *limit,
            //if limit is missing, fetch 10 subjects from register
            None => 10,
        })
        .and(with_handler(handler.clone()))
        .then(|limit: u64, handler: Arc<Handler>| async move {
            let result = handle_get_subjects(handler.clone(), limit).await;
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

    let get_subject_details = warp::path!("getSubjectDetails")
        .and(warp::get())
        .and(warp::query::<HashMap<String, i64>>())
        .map(|param: HashMap<String, i64>| match param.get("oib") {
            Some(oib) =>  *oib,
            None => {println!("OIB parameter is missing, returning 0"); 0},
        })
        .and(with_handler(handler))
        .then(|oib: i64, handler: Arc<Handler>| async move {
            let result = handle_subject_details(handler, oib).await;
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

    get_subjects.or(get_subject_details)
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

async fn handle_create_subject(handler: Arc<Handler>, register_details: &RegisterDetails) -> DocumentCreatedResult {
    handler.create_subject(map_details(register_details.clone())).await
}

async fn handle_get_subject_list(handler: Arc<Handler>, limit: Option<u64>) -> CouchResult<DocumentCollection<Subject>> {
    handler.get_subject_list(limit).await
}

async fn handle_get_details_from_db(handler: Arc<Handler>, oib: i64) -> Result<DocumentCollection<Details>, CouchError> {
    handler.get_details(oib).await
}

async fn handle_subject_details(handler: Arc<Handler>, oib: i64) -> Result<Details, Error> {
    let details_from_db: Result<DocumentCollection<Details>, CouchError> = handle_get_details_from_db(handler.clone(), oib).await;
    // check if result from db has data
    if !details_from_db.as_ref().unwrap().rows.is_empty() {
        Ok(details_from_db.unwrap().rows.into_iter().next().unwrap())
    } else {
        let details_from_register = get_subject_details(oib).await;
        match details_from_register {
            Ok(details) => {
                if let Some(details) = details {
                    let created_subject = handle_create_subject(handler, &details).await;
                    if let Err(err) = created_subject {
                        Err(Error { message: format!("Error while creating a subject: {:?}", err) })
                    } else {
                        Ok(map_details(details))
                    }
                } else {
                    Err(Error { message: "No entry found for given OIB".to_string() })
                }
            },
            Err(_err) => Err(Error { message: "Error getting details from register".to_string() })
        }
    }
}

async fn handle_get_subjects(handler: Arc<Handler>, limit: u64) -> Result<Vec<Subject>, Error> {
    let mut result: Vec<Subject> = Vec::new();
    let subjects_from_db = handle_get_subject_list(handler.clone(), Some(limit)).await;
    let mut db_subjects: Vec<Subject> = subjects_from_db.unwrap().rows;
    result.append(&mut db_subjects);
    let diff = limit - result.len() as u64;
    if diff > 0 {
        let subjects_from_register = get_subjects_from_register(result.len(), diff).await;
        println!("Limit exceeds the number of subjects in db by: {}", subjects_from_register.len());
        let mut new_subjects = map_subjects_from_register(subjects_from_register.as_ref());
        //write new subjects in db in a separate job
        tokio::spawn(async move {
            write_new_subjects_to_db(handler, subjects_from_register).await
        });
        //add new subjects from register to Vec<Subject> for return
        result.append(&mut new_subjects);
        Ok(result)
    } else {
        Ok(result)
    }
}

async fn write_new_subjects_to_db(handler: Arc<Handler>, subjects: Vec<RegisterSubject>) {
    for subject in subjects {
        match handle_create_subject(handler.clone(), &RegisterDetails { mbs: 0, oib: subject.oib }).await {
            Ok(created_result) => {
                println!("Subject with oib {} created, id: {}", subject.oib, created_result.id);
            },
            Err(err) => println!("Error creating subject with oib: {}, error: {:?}", subject.oib, err)
        }
    }
}