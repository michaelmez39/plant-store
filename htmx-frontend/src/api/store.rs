use axum::{extract::State, Json};
use serde::Serialize;
use store_lib::store::Product;
use ts_rs::TS;

use crate::AppState;

#[derive(Serialize, TS)]
#[ts(export)]
pub(crate) struct ListingResponse {
    listings: Vec<Product>,
}

pub(crate) async fn listing(
    State(AppState {
        inventory_backend, ..
    }): State<AppState>,
) -> Json<ListingResponse> {
    let listings = inventory_backend
        .products
        .lock()
        .unwrap()
        .values()
        .map(Product::clone)
        .collect();

    Json(ListingResponse { listings })
}
