use crate::Auth;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use store_lib::account::{Credentials, User};
use tracing::warn;
use ts_rs::TS;

#[derive(Serialize, TS)]
#[ts(export)]
enum LoginStatus {
    LoggedIn,
    NotLoggedIn,
    Unauthorized,
    Failed,
}

#[derive(Serialize, TS)]
#[ts(export)]
pub(crate) struct LoginResponse {
    status: LoginStatus,
    user: Option<User>,
}

pub(crate) async fn check_in(auth: Auth) -> Json<LoginResponse> {
    let status = match auth.user {
        Some(_) => LoginStatus::LoggedIn,
        None => LoginStatus::NotLoggedIn,
    };

    Json(LoginResponse {
        status,
        user: auth.user,
    })
}

pub(crate) async fn login(
    mut auth: Auth,
    Json(creds): Json<Credentials>,
) -> (StatusCode, Json<LoginResponse>) {
    let Ok(Some(user)) = auth.authenticate(creds).await else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(LoginResponse {
                status: LoginStatus::Unauthorized,
                user: None,
            }),
        );
    };

    match auth.login(&user).await {
        Ok(_) => (
            StatusCode::OK,
            Json(LoginResponse {
                status: LoginStatus::LoggedIn,
                user: Some(user),
            }),
        ),
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json(LoginResponse {
                status: LoginStatus::Failed,
                user: None,
            }),
        ),
    }
}

pub(crate) async fn logout(mut auth: Auth) -> Json<LoginResponse> {
    if let Err(e) = auth.logout().await {
        warn!("{}", e.to_string());
    }

    Json(LoginResponse {
        status: LoginStatus::NotLoggedIn,
        user: None,
    })
}
