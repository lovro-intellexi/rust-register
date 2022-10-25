/* 
use warp::{Filter, hyper::Response};
use std::collections::HashMap;

use model::{Subject};

async fn subject_request(limit: HashMap<String, u32>) -> String {
    let client = reqwest::Client::new();
    let res = client.get(format!("https://sudreg-api.pravosudje.hr/javni/subjekt/?offset=0&limit={}", limit.get("limit").unwrap()))
        .header("Ocp-Apim-Subscription-Key", "fd2756eee54b4b25b59b586a9185ea3b")
        .send()
        .await
        .expect("Error while sending a response");
    //let res = reqwest::get(format!("https://sudreg-api.pravosudje.hr/javni/subjekt/?offset=0&limit={}", limit.get("limit").unwrap())).await.expect("Error accessing API endpoint");
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await.expect("Error reading response body");
    println!("Body:\n{}", body);
    body
}

let _subject = warp::get()
        .and(warp::path("getSubject"))
        .and(warp::query::<HashMap<String, u32>>())
        .map(|p: HashMap<String, u32>| match p.get("limit") {
            //TODO call subject_request
            Some(limit) => Response::builder().body(subject_request),
            None => Response::builder().body(String::from("No \"limit\" param in query.")),
        });


     let subject_path = warp::path(base_path).and(warp::path("getSubject")).and(warp::query::<HashMap<String, u32>>())
        .map(|p: HashMap<String, u32>| match p.get("limit") {
            Some(limit) => Response::builder().body(format!("subject_request with limit: {}", limit).to_string()),
            None => Response::builder().body(String::from("No \"limit\" param in query.")),
        });
*/