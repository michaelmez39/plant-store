use std::collections::HashMap;

use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use store_lib::cart::{Cart, CartItem};
use ts_rs::TS;
use uuid::Uuid;

use crate::{AppState, Auth};
use axum_login::AuthUser;

use super::StoreError;

pub(crate) async fn fetch_cart(
    auth: Auth,
    State(AppState { cart_backend, .. }): State<AppState>,
) -> Result<Json<Cart>, StoreError> {
    let mut cart_store = cart_backend
        .lock()
        .map_err(|e| StoreError::internal(e.to_string()))?;

    // TODO: Allow viewing cart without user?
    let user = auth.user.ok_or(StoreError::unauthorized(
        "Must be logged in to view cart".to_string(),
    ))?;

    let Some(cart) = cart_store.carts.get(&user.id) else {
        cart_store.carts.insert(user.id, Cart::default());
        return Ok(Json(Cart::default()));
    };

    Ok(Json(cart.clone()))
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct AddToCart {
    pub number: usize,
}

pub(crate) async fn add_to_cart(
    auth: Auth,
    State(AppState {
        cart_backend,
        inventory_backend,
        ..
    }): State<AppState>,
    Path(listing_id): Path<Uuid>,
    Json(item): Json<AddToCart>,
) -> Result<Json<Cart>, StoreError> {
    let mut cart_store = cart_backend
        .lock()
        .map_err(|e| StoreError::internal(e.to_string()))?;

    let inventory = inventory_backend
        .products
        .lock()
        .map_err(|e| StoreError::internal(e.to_string()))?;

    let user_id = auth
        .user
        .ok_or(StoreError::unauthorized("user not found".to_string()))?
        .id();

    let cart = cart_store
        .carts
        .entry(user_id)
        .or_insert_with(|| Cart::new(HashMap::new()));

    let listing = inventory
        .get(&listing_id)
        .ok_or(StoreError::missing_inventory(listing_id.to_string()))?;

    cart.items
        .entry(listing_id)
        .and_modify(|p| p.number += item.number)
        .or_insert(CartItem {
            listing: listing.clone(),
            number: item.number,
        });

    Ok(Json(cart.clone()))
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RemoveFromCart {
    pub listing_id: Uuid,
}

pub(crate) async fn remove_from_cart(
    auth: Auth,
    State(AppState { cart_backend, .. }): State<AppState>,
    Path(listing_id): Path<Uuid>,
) -> Result<Json<Cart>, StoreError> {
    let mut cart_store = cart_backend
        .lock()
        .map_err(|e| StoreError::internal(e.to_string()))?;

    let user_id = auth
        .user
        .ok_or(StoreError::unauthorized("user not found".to_string()))?
        .id();

    let cart = cart_store
        .carts
        .entry(user_id)
        .or_insert_with(|| Cart::new(HashMap::new()));

    cart.items.remove(&listing_id);

    Ok(Json(cart.clone()))
}
