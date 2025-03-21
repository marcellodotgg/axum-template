use std::collections::HashMap;
use std::env;
use axum::body::Body;
use axum::extract::{Query, State};
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use sqlx::Error::RowNotFound;
use crate::AppState;
use crate::auth::GoogleOAuthClient;
use crate::user::User;
use crate::utils::cookies::Cookies;
use crate::utils::jwt::Jwt;

pub async fn oauth() -> impl IntoResponse {
    let client = GoogleOAuthClient::new();
    client.authenticate()
}

pub async fn oauth_callback(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let redirect_success_url = env::var("POST_LOGIN_REDIRECT_URL").expect("POST_LOGIN_REDIRECT_URL must be set");
    let redirect_error_url = env::var("POST_LOGIN_REDIRECT_ERROR_URL").expect("POST_LOGIN_REDIRECT_ERROR_URL must be set");

    let client = GoogleOAuthClient::new();

    // Handle missing 'code' params
    let code = match params.get("code") {
        Some(code) => code,
        None => return redirect(redirect_error_url),
    };

    // Exchange the code for a token
    let token = match client.clone().exchange_code_for_token(code).await {
        Ok(token) => token,
        Err(_) => return redirect(redirect_error_url),
    };

    // Fetch user information from Google
    let google_user = match client.clone().get_user_from_token(&token).await {
        Ok(google_user) => google_user,
        Err(_) => return redirect(redirect_error_url),
    };

    let jwt = match Jwt::encode(&google_user.email) {
        Ok(jwt) => jwt,
        Err(_) => return redirect(redirect_error_url),
    };

    // at this point, we know a user exists, so we should create an account if they don't have one
    if let Err(RowNotFound) = User::find_by_email(&state.db, &google_user.email).await {
        if let Err(err) = User::from_google(google_user).create(&state.db).await {
            println!("Something went wrong: {}", err);
            return redirect(redirect_error_url)
        }
    }

    Response::builder()
        .status(StatusCode::FOUND)
        .header("Set-Cookie", Cookies::new("auth_token", &jwt))
        .header("Location", redirect_success_url)
        .body(Body::empty())
        .unwrap()
}

fn redirect(url: String) -> Response<Body> {
    Response::builder().status(StatusCode::FOUND).header("Location", url).body(Body::empty()).unwrap()
}