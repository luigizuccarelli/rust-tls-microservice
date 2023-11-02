use warp::{reply::json, Rejection, Reply};

use crate::schema::*;

/// main handler - reads json payload
pub async fn process_payload(req: VSBase) -> Result<impl Reply, Rejection> {
    Ok(json(&VSResponse::res(req)))
}

/// health check handler
pub async fn health_handler() -> Result<impl Reply, Rejection> {
    let res = IsAlive {
        name: "rust-tls-microservice".to_string(),
        version: "0.0.1".to_string(),
        status: "ok".to_string(),
    };
    Ok(json(&res))
}
