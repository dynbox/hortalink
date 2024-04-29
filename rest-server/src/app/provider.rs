use oauth2::{AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl};
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::reqwest::async_http_client;
use oauth2::url::Url;
use serde::de::DeserializeOwned;

use common::settings::secrets::{OauthSecret, Secrets};

#[derive(Clone)]
pub struct OAuthProvider {
    pub google: Provider,
    pub facebook: Provider,
    pub linkedin: Provider,
}

#[derive(Clone)]
pub struct Provider {
    pub client: BasicClient,
    pub info: String,
    scopes: Vec<Scope>,
}

impl OAuthProvider {
    pub fn new(secrets: &Secrets, host: String) -> Self {
        Self {
            google: Provider::new(
                Self::oauth_client(
                    &secrets.google,
                    "https://accounts.google.com/o/oauth2/auth",
                    "https://accounts.google.com/o/oauth2/token",
                    format!("{host}/api/v1/oauth/google/callback"),
                ),
                "https://www.googleapis.com/oauth2/v3/userinfo".to_string(),
                vec!["email", "profile"],
            ),
            facebook: Provider::new(
                Self::oauth_client(
                    &secrets.facebook,
                    "https://www.facebook.com/dialog/oauth",
                    "https://graph.facebook.com/oauth/access_token",
                    format!("{host}/api/v1/oauth/facebook/callback"),
                ),
                "https://graph.facebook.com/me?fields=name,email".to_string(),
                vec!["email", "public_profile"],
            ),
            linkedin: Provider::new(
                Self::oauth_client(
                    &secrets.linkedin,
                    "https://www.linkedin.com/oauth/v2/authorization",
                    "https://www.linkedin.com/oauth/v2/accessToken",
                    format!("{host}/api/v1/oauth/linkedin/callback"),
                ),
                "https://api.linkedin.com/v2/userinfo".to_string(),
                vec!["profile", "email", "openid"],
            ),
        }
    }

    fn oauth_client(
        secret: &OauthSecret,
        auth_url: &str,
        token_url: &str,
        redirect_url: String,
    ) -> BasicClient {
        BasicClient::new(
            ClientId::new(secret.client_id.to_string()),
            Some(ClientSecret::new(secret.client_secret.to_string())),
            AuthUrl::new(auth_url.to_string()).unwrap(),
            Some(TokenUrl::new(token_url.to_string()).unwrap()),
        )
            .set_redirect_uri(
                RedirectUrl::new(redirect_url).unwrap()
            )
    }

    pub fn get_provider(&self, oauth_type: &str) -> &Provider {
        match oauth_type {
            "google" => &self.google,
            "facebook" => &self.facebook,
            _ => &self.linkedin
        }
    }
}

impl Provider {
    pub fn new(client: BasicClient, info: String, scopes: Vec<&str>) -> Self {
        Self {
            client,
            info,
            scopes: scopes.iter().map(|s| Scope::new(s.to_string())).collect(),
        }
    }

    pub fn auth_url(&self) -> (Url, CsrfToken) {
        self.client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(self.scopes.clone())
            .url()
    }

    pub async fn get_token(&self, code: String) -> Result<BasicTokenResponse, ()> {
        Ok(
            self.client
                .exchange_code(AuthorizationCode::new(code))
                .request_async(async_http_client)
                .await
                .map_err(|_| ())?
        )
    }

    pub async fn get_user<T: DeserializeOwned>(&self, token: &String) -> T {
        reqwest::Client::new()
            .get(&self.info)
            .bearer_auth(token)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
}