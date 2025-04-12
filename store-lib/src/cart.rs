use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::store::Product;
use bigdecimal::{BigDecimal, num_bigint::BigInt};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Clone)]
pub struct CartBackend(Arc<Mutex<CartStore>>);

impl CartBackend {
    pub fn new() -> Self {
        CartBackend(Arc::new(Mutex::new(CartStore {
            carts: HashMap::new(),
        })))
    }
}

pub struct CartStore {
    pub carts: HashMap<Uuid, Cart>,
}

impl std::ops::Deref for CartBackend {
    type Target = Arc<Mutex<CartStore>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deserialize, Serialize, Clone, TS)]
pub struct CartItem {
    pub listing: Product,
    pub number: usize,
}

#[derive(Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct AddToCart {
    pub listing_id: Uuid,
    pub number: usize,
}

#[derive(Serialize, Clone, TS)]
pub struct Cart {
    pub items: Vec<CartItem>,
}

impl Default for Cart {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl Cart {
    pub fn new(items: Vec<CartItem>) -> Self {
        Self { items }
    }

    pub fn subtotal(&self) -> BigDecimal {
        self.items
            .iter()
            .fold(BigDecimal::new(BigInt::ZERO, 2), |acc, item| acc + item.listing.price.clone())
    }
}
