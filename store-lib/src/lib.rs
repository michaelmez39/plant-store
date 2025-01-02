pub mod account {
    use std::sync::{Arc, Mutex};

    use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
    use argon2::Argon2;
    use axum_login::tracing::info;
    use axum_login::{AuthUser, AuthnBackend, UserId};
    use rand::rngs::OsRng;
    use serde::Deserialize;
    use uuid::Uuid;

    #[derive(Clone, Debug)]
    pub struct User {
        pub id: Uuid,
        pub email: String,
        pub username: String,
        password: String,
    }

    #[derive(Debug)]
    pub enum UserError {
        PasswordHashingFailed(argon2::password_hash::Error),
        NotFound,
    }

    impl std::fmt::Display for UserError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let error = match self {
                UserError::NotFound => "not found",
                UserError::PasswordHashingFailed(_) => "error",
            };

            writeln!(f, "{}", error)
        }
    }

    impl std::error::Error for UserError {}

    impl From<argon2::password_hash::Error> for UserError {
        fn from(value: argon2::password_hash::Error) -> Self {
            UserError::PasswordHashingFailed(value)
        }
    }

    impl User {
        fn new(username: String, email: String, password: String) -> Result<Self, UserError> {
            let argon2 = Argon2::default();
            let salt = SaltString::generate(&mut OsRng);
            let password = argon2.hash_password(password.as_bytes(), &salt)?;
            info!("Made new user");
            Ok(User {
                id: Uuid::new_v4(),
                username,
                email,
                password: password.to_string(),
            })
        }

        fn authenticate(&self, password: String) -> Result<(), UserError> {
            let password_hash = PasswordHash::new(&self.password)?;
            Argon2::default().verify_password(password.as_bytes(), &password_hash)?;
            Ok(())
        }
    }

    impl AuthUser for User {
        type Id = Uuid;

        fn id(&self) -> Self::Id {
            self.id
        }

        fn session_auth_hash(&self) -> &[u8] {
            self.password.as_bytes()
        }
    }

    #[derive(Clone, Deserialize)]
    pub struct Credentials {
        pub email: String,
        pub password: String,
    }

    impl From<Signup> for Credentials {
        fn from(value: Signup) -> Self {
            Credentials {
                email: value.email.clone(),
                password: value.password.clone(),
            }
        }
    }

    #[derive(Deserialize, Clone)]
    pub struct Signup {
        pub email: String,
        pub password: String,
        pub username: String,
    }

    #[derive(Clone)]
    pub struct UserStore {
        users: Vec<User>,
    }

    #[derive(Clone)]
    pub struct UserBackend(pub Arc<Mutex<UserStore>>);

    #[axum_login::axum::async_trait]
    impl AuthnBackend for UserBackend {
        type User = User;
        type Credentials = Credentials;
        type Error = UserError;

        async fn authenticate(
            &self,
            Self::Credentials { email, password }: Self::Credentials,
        ) -> Result<Option<Self::User>, Self::Error> {
            let user_store = self.0.lock().expect("user backend threads");
            let user = user_store
                .users
                .iter()
                .find(|u| u.email == email)
                .ok_or(UserError::NotFound)?;

            match user.authenticate(password) {
                Ok(_) => Ok(Some(user.clone())),
                Err(UserError::NotFound) => Ok(None),
                Err(e) => Err(e),
            }
        }

        async fn get_user(
            &self,
            user_id: &UserId<Self>,
        ) -> Result<Option<Self::User>, Self::Error> {
            let user_store = self.0.lock().expect("user backend threads");
            Ok(user_store
                .users
                .iter()
                .find(|u| u.id == *user_id)
                .map(|u| u.clone()))
        }
    }

    pub enum LoginError {
        InvalidPassword,
        InvalidUsername,
    }

    impl UserStore {
        pub fn new() -> Self {
            UserStore { users: Vec::new() }
        }

        pub fn add(&mut self, signup: Signup) -> Result<(), UserError> {
            let user = User::new(signup.username, signup.email, signup.password)?;
            self.users.push(user);
            info!("Users {:?}", self.users);
            Ok(())
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
