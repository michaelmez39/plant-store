use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use axum_login::AuthSession;
use maud::{html, Markup};
use store_lib::account::{Credentials, Signup, UserBackend};
use tracing::{info, warn};

use crate::{components::PageWrapper, AppState, Auth};

pub async fn login(page: PageWrapper) -> Markup {
    page.render(login_page(login_form()))
}

pub async fn login_post(mut auth: Auth, Form(creds): Form<Credentials>) -> Redirect {
    let Ok(Some(user)) = auth.authenticate(creds).await else {
        return Redirect::to("/login");
    };

    match auth.login(&user).await {
        Ok(_) => Redirect::to("/"),
        Err(_) => Redirect::to("/login"),
    }
}

pub fn login_page(content: Markup) -> Markup {
    html! {
        .container.my-6 {
            .columns {
                .column.is-one-third {
                    .box {
                        (content)
                    }
                }
            }
        }
    }
}

fn login_form() -> Markup {
    html!(
        form method="post" action="/login" {
            h2.is-size-3 {
                "Sign In"
            }
            .field  {
                label.label for="email" { "Email" }
                .control {
                    input.input name="email" id="email" {}
                }
            }
            .field {
                label.label for="password" { "Password" }
                .control {
                    input.input name="password" id="password" type="password" {}
                }
            }

            button.button.is-link {
                "Login"
            }
        }
    )
}

pub async fn create_account(page: PageWrapper) -> Markup {
    page.render(login_page(create_account_form()))
}

pub fn add_user(store: UserBackend, signup: Signup) -> Result<(), store_lib::account::UserError> {
    let mut user_store = store.0.lock().expect("user store threading issue");
    user_store.add(signup.clone())
}

#[axum_macros::debug_handler]
pub async fn create_account_post(
    State(s): State<AppState>,
    mut auth: AuthSession<UserBackend>,
    Form(signup): Form<Signup>,
) -> impl IntoResponse {
    info!("Creating account for: {}", signup.email);
    let Ok(_) = add_user(s.user_store, signup.clone()) else {
        warn!("failed to store new user");
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    info!("Authenticating user");
    let auth_result = match auth.authenticate(signup.into()).await {
        Ok(Some(ref u)) => auth.login(u).await,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_result.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    Redirect::to("/").into_response()
}

fn create_account_form() -> Markup {
    html!(
        form method="post" action="/signup" {
            h2.is-size-3 {
                "Create Account"
            }
            .field  {
                label.label for="username" { "Username" }
                .control {
                    input.input name="username" id="username" {}
                }
            }
            .field {
                label.label for="email" { "Email" }
                .control {
                    input.input name="email" id="email" type="email" {}
                }
            }
            .field {
                label.label for="password" { "Password" }
                .control {
                    input.input name="password" id="password" type="password" {}
                }
            }
            button.button.is-link {
                "Create"
            }
        }
    )
}

pub async fn logout(mut auth: Auth) -> impl IntoResponse {
    match auth.logout().await {
        Ok(_) => Redirect::to("/login").into_response(),
        Err(e) => logout_error(&auth, e).into_response(),
    }
}

fn logout_error(auth: &Auth, error: axum_login::Error<UserBackend>) -> Markup {
    let user = auth.user.as_ref().map(|u| u.email.as_str()).unwrap_or("");
    warn!("Logout failed for user {user} with error {error}");
    html! {
        div {
            h3 { "Logout Failed" }
        }
    }
}
