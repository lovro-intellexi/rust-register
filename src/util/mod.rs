use std::sync::Arc;

use warp::Filter;

use crate::{handler::handler::Handler, model::{RegisterSubject, Subject}};

pub fn with_handler(handler: Arc<Handler>) -> impl Filter<Extract = (Arc<Handler>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || handler.clone())
  }

pub async fn handle_subjects_from_register(limit: String) -> Vec<RegisterSubject> {
  let reqwest_client = reqwest::Client::new();
  let result = reqwest_client.get(format!("https://sudreg-api.pravosudje.hr/javni/subjekt/?offset=0&limit={}", limit))
      .header("Ocp-Apim-Subscription-Key", "fd2756eee54b4b25b59b586a9185ea3b")
      .send()
      .await
      .expect("failed to get a response")
      .json::<Vec<RegisterSubject>>()
      .await;
  match result {
    Ok(register_subjects) => register_subjects,
    Err(_err) => Vec::new()
  }
}

pub async fn check_db_for_new_subjects(subjects: Vec<RegisterSubject>) {
  let db_subjects = map_subjects(subjects);
  println!("{:?}", db_subjects);
}

fn map_subjects(subjects: Vec<RegisterSubject>) -> Vec<Subject> {
  let mut db_subjects = Vec::new();
  for subject in subjects {
    let db_subject = Subject {
      _id: "".to_string(),
      _rev: "".to_string(),
      oib: subject.oib,
      name: "".to_string()
    };
    db_subjects.push(db_subject);
  }
  db_subjects
}