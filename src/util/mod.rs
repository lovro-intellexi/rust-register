use std::sync::Arc;

use warp::Filter;

use crate::handler::handler::Handler;

pub fn with_handler(handler: Arc<Handler>) -> impl Filter<Extract = (Arc<Handler>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || handler.clone())
  }