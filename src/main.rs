use crate::schema::*;
use clap::Parser;
use warp::Filter;

mod errors;
mod handlers;
mod schema;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let health = warp::path("isalive");
    let health_route = health.and_then(handlers::health_handler);

    let process = warp::path("process");
    let process_route = process
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handlers::process_payload);

    let routes = health_route
        .or(process_route)
        .with(warp::cors().allow_any_origin())
        .recover(errors::handle_rejection);

    // start the service
    if args.tls {
        println!("starting service with tls");
        warp::serve(routes)
            .tls()
            .cert_path("./certs/domain.crt")
            .key_path("./certs/domain.key")
            .run(([0, 0, 0, 0], 9000))
            .await;
    } else {
        println!("starting service with no tls");
        warp::serve(routes).run(([0, 0, 0, 0], 9000)).await;
    }
}
