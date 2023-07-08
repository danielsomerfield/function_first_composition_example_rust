use axum::{Json, Router};
use axum::routing::{get};
use serde::Serialize;
use tokio::spawn;

#[derive(Serialize)]
struct Response {
    status: String
}


async fn get_it() -> Json<Response> {
    return Json(Response {
        status: "OK".to_string(),
    });
}

pub async fn start() -> Server {
    let app: Router = Router::new().route(
        "/vancouverbc/restaurants/recommended",
        get(get_it),
    );

    let server = axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service());
    spawn(async {
        server.await.expect("Server didn't start");
        println!("Started");
    });


    return Server {};
}

pub struct Server {
}

impl Server {
    pub fn stop(self) {}
}