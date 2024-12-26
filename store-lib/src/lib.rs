pub mod account {
    use uuid::Uuid;

    struct User {
        pub id: Uuid,
        pub username: String,
        pub password: String,
    }

    impl User {
        fn new(name: String, password: String) -> Self {
            User {
                id: Uuid::new_v4(),
                username: name,
                password,
            }
        }
    }

    pub struct UserStore {
        users: Vec<User>,
    }

    pub enum LoginError {
        InvalidPassword,
        InvalidUsername,
    }

    impl UserStore {
        pub fn new() -> Self {
            UserStore { users: Vec::new() }
        }

        pub fn add(&mut self, username: String, password: String) {
            self.users.push(User::new(username, password))
        }

        pub fn validate(&self, username: String, password: String) -> Result<(), LoginError> {
            let Some(stored_user) = self.users.iter().find(|u| u.username == username) else {
                return Err(LoginError::InvalidUsername);
            };

            if stored_user.password != password {
                return Err(LoginError::InvalidPassword);
            }

            Ok(())
        }
    }
}

pub mod cart {
    use crate::store::ItemListing;
    use rust_decimal::Decimal;

    pub struct Cart {
        pub items: Vec<CartItem>,
    }

    pub struct CartItem {
        pub listing: ItemListing,
        pub number: usize,
    }

    impl Cart {
        pub fn subtotal(&self) -> Decimal {
            self.items
                .iter()
                .fold(Decimal::ZERO, |acc, item| acc + item.listing.price)
        }
    }
}

pub mod store {
    use rand::distributions::Uniform;
    use rand::prelude::*;
    use rust_decimal::Decimal;
    use sqlx::types::Uuid;

    pub struct ItemListing {
        pub listing_id: Uuid,
        pub name: String,
        pub price: Decimal,
        pub description: String,
        pub image: String,
    }

    impl ItemListing {
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
            let price = Decimal::new(rng.sample(Uniform::new(100, 3000)), 2);
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
}
