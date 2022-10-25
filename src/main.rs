use std::sync::Arc;

use db::RegisterAdapter;
use handler::handler::Handler;
use web::WebServer;

use crate::{db::init_db};

mod db;
mod web;
mod model;
mod handler;
mod util;

const DEFAULT_WEB_PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    let web_port = DEFAULT_WEB_PORT;

    //init db
    let db = init_db().await.expect("Init db failed");
    let db = Arc::new(db);

    //init register adapter
    let reg_adapter = Arc::new(RegisterAdapter::new(db));

    //init handler
    let handler = Arc::new(Handler::new(reg_adapter));

    //init web server
    let web_server = Arc::new(WebServer::new(handler));

    //start web server
    match web_server.start_server(web_port).await {
        Ok(_) => println!("Server ended"),
        Err(ex) => println!("Web server failed to start: {:?}", ex)
    }
}
