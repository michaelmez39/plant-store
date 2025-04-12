mod account;
mod cart;
mod store;

use crate::AppState;
use account::{check_in, login, logout};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use cart::{add_to_cart, fetch_cart};
use serde::Serialize;
use store::listing;

#[derive(Serialize)]
enum ErrorCause {
    Internal,
    Unauthorized,
    MissingInventory,
}

#[derive(Serialize)]
struct ApiError {
    reason: ErrorCause,
    message: String,
}

impl ApiError {
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

impl IntoResponse for ApiError {
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
        .route("/cart", post(add_to_cart))
}
