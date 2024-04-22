mod components;
mod pages;

use axum::routing::put;
use axum::{routing::get, Router};
use components::{notification, Color};
use maud::Markup;
use rand::distributions::Uniform;
use rand::prelude::*;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::info;
use uuid::Uuid;
use crate::pages::shopping::shopping;
use crate::pages::store::{store, rock_list, add_to_cart};

const PORT: &'static str = "3000";

struct ItemListing {
    listing_id: Uuid,
    name: String,
    price: usize,
    image: String,
}

impl ItemListing {
    fn random() -> Self {
        let mut rng = thread_rng();
        let name = vec![
            "Amethyst",
            "Pothos",
            "Ruby",
            "Aroid",
            "Garnet",
            "Diamond",
            "Succulent",
        ]
        .choose(&mut rng)
        .expect("Hard coded list has items")
        .to_string();

        let image = vec![
            "amethyst.jpg",
            "blue.jpg",
            "blue_rock.jpg",
            "Chalcanthite.webp",
            "quartz.jpg",
            "talc.webp",
            "pothos.jpg",
        ]
        .choose(&mut rng)
        .expect("Hard coded list has items")
        .to_string();

        let listing_id = Uuid::new_v4();
        let price = rng.sample(Uniform::new(100, 3000));
        Self {
            name,
            listing_id,
            price,
            image,
        }
    }
}

async fn trouble() -> Markup {
    notification("Causing Trouble", Color::Default, false).await
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let pages = Router::new()
        .route("/trouble", get(trouble))
        .route("/rock-list/:id", get(rock_list))
        .route("/add-to-cart/:id", put(add_to_cart))
        .route("/", get(store))
        .route("/shopping-cart", get(shopping))
        .layer(TraceLayer::new_for_http())
        .nest_service("/assets", ServeDir::new("assets").fallback(ServeFile::new("assets/amethyst.jpg")));

    info!("Server started on port {}", PORT);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", PORT))
        .await
        .unwrap();
    axum::serve(listener, pages).await.unwrap();
}
