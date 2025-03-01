use std::env;
use axum::response::Redirect;
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl};
use oauth2::basic::BasicClient;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GoogleUser {
    pub email: String,
    pub picture: String,
    pub given_name: String,
    pub family_name: String,
}

#[derive(Clone)]
pub struct GoogleOAuthClient {
    auth_url: AuthUrl,
    token_url: TokenUrl,
    redirect_url: RedirectUrl,
    client_secret: ClientSecret,
    client_id: ClientId
}

impl GoogleOAuthClient {
    pub fn new() -> Self {
        Self {
            client_id: ClientId::new(env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not found")),
            client_secret: ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET not found")),
            redirect_url: RedirectUrl::new(env::var("GOOGLE_CLIENT_REDIRECT").expect("GOOGLE_CLIENT_REDIRECT not found")).unwrap(),
            token_url: TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap(),
            auth_url: AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string()).unwrap()
        }
    }

    pub fn authenticate(self) -> Redirect {
        let client = BasicClient::new(self.client_id)
            .set_client_secret(self.client_secret)
            .set_auth_uri(self.auth_url)
            .set_token_uri(self.token_url)
            .set_redirect_uri(self.redirect_url);

        let (auth_url, _) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .url();

        Redirect::to(auth_url.as_str())
    }

    pub async fn exchange_code_for_token(self, code: &str) -> Result<String, String> {
        let client = BasicClient::new(self.client_id)
            .set_client_secret(self.client_secret)
            .set_auth_uri(self.auth_url)
            .set_token_uri(self.token_url)
            .set_redirect_uri(self.redirect_url);

        let async_http_client = reqwest::ClientBuilder::new().redirect(oauth2::reqwest::redirect::Policy::none()).build().expect("Client should build");

        let token_result = client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request_async(&async_http_client)
            .await;

        match token_result {
            Ok(token) => Ok(token.access_token().secret().clone()),
            Err(err) => Err(err.to_string())
        }
    }

    pub async fn get_user_from_token(self, token: &str) -> Result<GoogleUser, oauth2::reqwest::Error> {
        let response = reqwest::Client::new()
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(token)
            .send()
            .await?
            .json::<GoogleUser>()
            .await?;
        Ok(response)
    }
}
