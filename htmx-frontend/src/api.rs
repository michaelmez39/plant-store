mod account;
mod cart;
mod store;

use crate::AppState;
use account::{check_in, login, logout};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use cart::{add_to_cart, fetch_cart, remove_from_cart};
use serde::Serialize;
use store::listing;
use ts_rs::TS;

#[derive(Serialize, TS)]
enum ErrorCause {
    Internal,
    Unauthorized,
    MissingInventory,
}

#[derive(Serialize, TS)]
#[ts(export)]
struct StoreError {
    reason: ErrorCause,
    message: String,
}

impl StoreError {
    fn internal(message: String) -> Self {
        Self {
            reason: ErrorCause::Internal,
            message,
        }
    }

    fn unauthorized(message: String) -> Self {
        Self {
            reason: ErrorCause::Unauthorized,
            message,
        }
    }

    fn missing_inventory(message: String) -> Self {
        Self {
            reason: ErrorCause::MissingInventory,
            message,
        }
    }
}

impl IntoResponse for StoreError {
    fn into_response(self) -> axum::response::Response {
        let code = match self.reason {
            ErrorCause::Internal => StatusCode::BAD_REQUEST,
            ErrorCause::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrorCause::MissingInventory => StatusCode::NOT_FOUND,
        };
        (code, Json(self)).into_response()
    }
}

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/status", get(async || "alive"))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/check-in", get(check_in))
        .route("/listings", get(listing))
        .route("/cart", get(fetch_cart))
        .route("/cart/{listing_id}", post(add_to_cart))
        .route("/cart/{listing_id}", delete(remove_from_cart))
}

#[cfg(test)]
mod test {
    #[test]
    fn export_typescript() {}
}
