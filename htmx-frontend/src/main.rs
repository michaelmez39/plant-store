mod api;
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
use store_lib::account::UserBackend;
use store_lib::cart::CartBackend;
use store_lib::store::{Inventory, InventoryBackend, Product};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::info;

const PORT: &'static str = "8080";

pub type Auth = AuthSession<UserBackend>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = option_env!("PORT")
        .map(str::to_string)
        .or_else(|| std::env::args().skip(1).next())
        .unwrap_or(String::from(PORT));

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Asset Service
    let asset_service = ServeDir::new("assets");
    let user_backend = UserBackend::new();
    let cart_backend = CartBackend::new();
    let inventory_backend = InventoryBackend::new();

    // Add testing user for testing
    user_backend
        .lock()
        .expect("users available")
        .add(store_lib::account::Signup {
            email: "michael@example.com".to_string(),
            password: "password".to_string(),
            username: "michael".to_string(),
        })?;

    // Add Some Inventory
    {
        let mut products = inventory_backend
            .products
            .lock()
            .expect("products available");

        let mut inventory = inventory_backend
            .inventory
            .lock()
            .expect("inventory available");

        for listing in (0..10).map(|_| Product::random()) {
            inventory.insert(listing.listing_id, Inventory::new(12, 3, 15));
            products.insert(listing.listing_id, listing);
        }
    }

    // Auth service.
    let session_store = axum_login::tower_sessions::MemoryStore::default();
    let session_layer = axum_login::tower_sessions::SessionManagerLayer::new(session_store);
    let auth_layer = AuthManagerLayerBuilder::new(user_backend.clone(), session_layer).build();

    // App State
    let state = AppState {
        user_backend,
        cart_backend,
        inventory_backend,
    };

    let api_routes = api::api_routes();

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
        .nest("/api", api_routes)
        .layer(TraceLayer::new_for_http())
        .layer(auth_layer)
        .nest_service("/assets", asset_service)
        .with_state(state);

    info!("Server started on port {port}");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    axum::serve(listener, pages).await.unwrap();
    Ok(())
}

#[derive(Clone)]
struct AppState {
    user_backend: UserBackend,
    cart_backend: CartBackend,
    inventory_backend: InventoryBackend,
}

async fn status() -> &'static str {
    "OK"
}
