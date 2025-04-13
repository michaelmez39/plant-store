use bigdecimal::BigDecimal;
use rand::distributions::Uniform;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Clone)]
pub struct InventoryBackend {
    pub inventory: Arc<Mutex<HashMap<Uuid, Inventory>>>,
    pub products: Arc<Mutex<HashMap<Uuid, Product>>>,
}
impl InventoryBackend {
    pub fn new() -> Self {
        Self {
            inventory: Arc::new(Mutex::new(HashMap::new())),
            products: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

pub struct Inventory {
    pub free: usize,
    pub ordered: usize,
    pub sent: usize,
}

impl Inventory {
    pub fn new(free: usize, ordered: usize, sent: usize) -> Self {
        Self {
            free,
            ordered,
            sent,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct Product {
    pub listing_id: Uuid,
    pub name: String,
    pub price: BigDecimal,
    pub description: String,
    pub image: String,
}

impl Product {
    pub fn random() -> Self {
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
        let price = BigDecimal::new(rng.sample(Uniform::new(100, 3000)).into(), 2);
        let description = "The description of the item we are looking at".to_string();

        Self {
            name,
            listing_id,
            price,
            image,
            description,
        }
    }
}
