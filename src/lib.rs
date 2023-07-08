use axum::Router;

pub mod server;
mod restaurant_ratings;

trait DiModule {
    fn routers(&self) -> Vec<Router>;
}