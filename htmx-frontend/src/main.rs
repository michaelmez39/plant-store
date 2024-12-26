mod components;
mod pages;
mod utils;

use crate::pages::checkout::checkout;
use crate::pages::shopping::shopping;
use crate::pages::store::{add_to_cart, rock_list, store};
use store_lib::account::UserStore;

use axum::routing::put;
use axum::{routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::info;

const PORT: &'static str = "3000";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let asset_service = ServeDir::new("assets").fallback(ServeFile::new("assets/amethyst.jpg"));

    let pages = Router::new()
        .route("/rock-list/:id", get(rock_list))
        .route("/add-to-cart/:id", put(add_to_cart))
        .route("/", get(store))
        .route("/shopping-cart", get(shopping))
        .route("/checkout", get(checkout))
        .layer(TraceLayer::new_for_http())
        .nest_service("/assets", asset_service)
        .with_state(state);

    info!("Server started on port {}", PORT);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", PORT))
        .await
        .unwrap();
    axum::serve(listener, pages).await.unwrap();
}

struct AppState {
    user: UserStore,
}
