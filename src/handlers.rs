use std::env;
use warp::{reply::json, Rejection, Reply};

use crate::schema::*;

/// main handler - reads json payload
pub async fn process_payload(req: VSBase) -> Result<impl Reply, Rejection> {
    Ok(json(&VSResponse::res(req)))
}

/// health check handler
pub async fn health_handler() -> Result<impl Reply, Rejection> {
    let res = IsAlive {
        name: env::var("HOSTNAME").unwrap(),
        version: "0.1.0".to_string(),
        status: "OK".to_string(),
    };
    Ok(json(&res))
}
