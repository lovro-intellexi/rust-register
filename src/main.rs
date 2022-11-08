use std::sync::Arc;

use db::RegisterAdapter;
use handler::Handler;
use web::WebServer;

use crate::db::init_db;

mod db;
mod web;
mod model;
mod handler;
mod util;

const DEFAULT_WEB_PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    //use when deplayed in environment
    //init env
    /*let env = init_env().await.expect("Init env failed");
    let env = Arc::new(env);
    let db = init_db(env).await.expect("Init db failed");
    let db = Arc::new(db)
    */

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
    match web_server.start_server(DEFAULT_WEB_PORT).await {
        Ok(_) => println!("Server ended"),
        Err(ex) => println!("Web server failed to start: {:?}", ex)
    }
}
