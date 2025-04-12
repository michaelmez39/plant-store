use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use store_lib::cart::{AddToCart, Cart, CartItem};
use uuid::Uuid;

use crate::{AppState, Auth};
use axum_login::AuthUser;

use super::ApiError;

pub(crate) async fn fetch_cart(
    auth: Auth,
    State(AppState { cart_backend, .. }): State<AppState>,
) -> Result<Json<Cart>, ApiError> {
    let mut cart_store = cart_backend
        .lock()
        .map_err(|e| ApiError::internal(e.to_string()))?;

    // TODO: Allow viewing cart without user?
    let user = auth.user.ok_or(ApiError::unauthorized(
        "Must be logged in to view cart".to_string(),
    ))?;

    let Some(cart) = cart_store.carts.get(&user.id) else {
        cart_store.carts.insert(user.id, Cart::default());
        return Ok(Json(Cart::default()));
    };

    Ok(Json(cart.clone()))
}

pub(crate) async fn add_to_cart(
    auth: Auth,
    State(AppState {
        cart_backend,
        inventory_backend,
        ..
    }): State<AppState>,
    Json(item): Json<AddToCart>,
) -> Result<Json<Cart>, ApiError> {
    let mut cart_store = cart_backend
        .lock()
        .map_err(|e| ApiError::internal(e.to_string()))?;

    let inventory = inventory_backend
        .products
        .lock()
        .map_err(|e| ApiError::internal(e.to_string()))?;

    let cart = cart_store
        .carts
        .entry(
            auth.user
                .ok_or(ApiError::unauthorized("user not found".to_string()))?
                .id(),
        )
        .or_insert_with(|| Cart::new(Vec::new()));

    let listing = inventory
        .get(&item.listing_id)
        .ok_or(ApiError::missing_inventory(item.listing_id.to_string()))?;

    cart.items.push(CartItem {
        listing: listing.clone(),
        number: item.number,
    });

    Ok(Json(cart.clone()))
}
