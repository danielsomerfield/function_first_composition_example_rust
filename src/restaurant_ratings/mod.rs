use crate::DiModule;
use axum::{Json, Router};
use axum::routing::get;
use serde::Serialize;

pub struct RestaurantRatingsDIModule {

}

#[derive(Serialize)]
struct Response {
    status: String,
}

async fn get_top_restaurants() -> Json<Response> {
    return Json(Response {
        status: "OK".to_string(),
    });
}

impl DiModule for RestaurantRatingsDIModule {
    fn routers(&self) -> Vec<Router> {
        let app: Router = Router::new().route(
            "/vancouverbc/restaurants/recommended",
            get(get_top_restaurants),
        );
        return vec![
            app
        ]
    }
}