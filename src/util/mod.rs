use std::sync::Arc;

use warp::Filter;

use crate::{handler::handler::Handler, model::{RegisterSubject, Subject, RegisterDetails, Details}};

pub fn with_handler(handler: Arc<Handler>) -> impl Filter<Extract = (Arc<Handler>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || handler.clone())
  }

pub async fn handle_subjects_from_register(limit: u64) -> Vec<RegisterSubject> {
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
    //TODO handle error
    Err(_err) => Vec::new()
  }
}

pub async fn handle_get_subject_details(oib: i64) -> Result<Option<RegisterDetails>, reqwest::Error> {
  let reqwest_client = reqwest::Client::new();
  //test oib: 76860791838
  let temp = reqwest_client.get(format!("https://sudreg-api.pravosudje.hr/javni/subjekt_detalji?tipIdentifikatora=oib&identifikator={}", oib))
    .header("Ocp-Apim-Subscription-Key", "fd2756eee54b4b25b59b586a9185ea3b")
    .send()
    .await
    .expect("failed to get a response")
    .json::<Option<RegisterDetails>>()
    .await;
  match temp {
      Ok(val) => {
        match val {
          Some(details) => Ok(Some(details)),
          None =>  Ok(val)
        }
      },
      Err(err) => Err(err)
  }
}

fn map_subjects(subjects: Vec<RegisterSubject>) -> Vec<Subject> {
  let mut db_subjects = Vec::new();
  for subject in subjects {
    let db_subject = Subject {
      _id: "".to_string(),
      _rev: "".to_string(),
      oib: subject.oib,
    };
    db_subjects.push(db_subject);
  }
  db_subjects
}

pub fn map_details(register_details: RegisterDetails) -> Details {
  Details {
    _id: "".to_string(),
    _rev: "".to_string(),
    mbs: register_details.mbs,
    oib: register_details.oib
  }
}