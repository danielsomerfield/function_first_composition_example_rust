use axum::{Json, Router};
use axum::routing::get;
use serde::Serialize;
use tokio::spawn;
use tokio::task::JoinHandle;

use crate::DiModule;
use crate::restaurant_ratings::RestaurantRatingsDIModule;

pub async fn start() -> JoinHandle<()> {
    let modules: Vec<Box<dyn DiModule>> = vec![Box::new(RestaurantRatingsDIModule {})];

    let app: Router = Router::new().route(
        "/healthcheck",
        get(|| async {
            Json(HealthResponse {
                status: "OK".to_string(),
            })
        }),
    );


    let merged_app = modules.iter()
        .flat_map(|m| m.routers())
        .fold(app, |r1, r2| r1.merge(r2));

    let server = axum::Server::bind(&"0.0.0.0:3000"
        .parse()
        .unwrap()
    ).serve(merged_app.into_make_service());

    return spawn(async {
        server.await.expect("Server didn't start");
        println!("Started");
    });
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}



