mod components;
mod pages;
mod utils;

use crate::pages::account::login;
use crate::pages::checkout::checkout;
use crate::pages::shopping::shopping;
use crate::pages::store::{add_to_cart, rock_list, store};
use axum::routing::{get, post, put};
use axum::Router;
use axum_login::{AuthManagerLayerBuilder, AuthSession};
use pages::account::{create_account, create_account_post, login_post, logout};
use std::sync::{Arc, Mutex};
use store_lib::account::{UserBackend, UserStore};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::info;

const PORT: &'static str = "3000";

pub type Auth = AuthSession<UserBackend>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Asset Service
    let asset_service = ServeDir::new("assets");
    let user_backend = UserBackend(Arc::new(Mutex::new(UserStore::new())));

    // Add testing user for testing
    user_backend.0.lock().unwrap().add(store_lib::account::Signup {
        email: "michael@example.com".to_string(),
        password: "$argon2id$v=19$m=19456,t=2,p=1$tbymO8omOx1Cv/JKctQ0LQ$NJzwR6JmStAZ75jGGKmz/aIAvYEPOArlWOANs4inxOY".to_string(),
        username: "michael".to_string()
    }).unwrap();

    // Auth service.
    let session_store = axum_login::tower_sessions::MemoryStore::default();
    let session_layer = axum_login::tower_sessions::SessionManagerLayer::new(session_store);
    let auth_layer = AuthManagerLayerBuilder::new(user_backend.clone(), session_layer).build();

    // App State
    let state = AppState {
        user_store: user_backend,
    };

    let pages = Router::new()
        .route("/rock-list/:id", get(rock_list))
        .route("/add-to-cart/:id", put(add_to_cart))
        .route("/", get(store))
        .route("/shopping-cart", get(shopping))
        .route("/checkout", get(checkout))
        .route("/login", get(login))
        .route("/login", post(login_post))
        .route("/logout", post(logout))
        .route("/signup", get(create_account))
        .route("/signup", post(create_account_post))
        .route("/status", get(status))
        .layer(TraceLayer::new_for_http())
        .layer(auth_layer)
        .nest_service("/assets", asset_service)
        .with_state(state);

    info!("Server started on port {}", PORT);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", PORT))
        .await
        .unwrap();
    axum::serve(listener, pages).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    user_store: UserBackend,
}

async fn status() -> &'static str {
    "OK"
}
