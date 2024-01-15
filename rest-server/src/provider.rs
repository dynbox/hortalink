use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::{AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl};
use oauth2::reqwest::async_http_client;
use oauth2::url::Url;
use serde::de::DeserializeOwned;
use common::config::secrets::OauthSecret;
use common::config::Settings;
use common::config::web::WebServerSettings;

#[derive(Debug, Clone)]
pub struct AuthProviders {
    pub google: Provider,
    pub facebook: Provider,
    pub linkedin: Provider,
}

#[derive(Debug, Clone)]
pub struct Provider {
    pub client: BasicClient,
    pub info: String,
    scopes: Vec<Scope>,
}

impl AuthProviders {
    pub fn new(settings: &Settings) -> Self {
        let secrets = &settings.secrets;

        Self {
            google: Provider {
                client: Self::create_provider(
                    &secrets.google,
                    "https://accounts.google.com/o/oauth2/auth",
                    "https://accounts.google.com/o/oauth2/token",
                    "/api/oauth/callback/google",
                    &settings.webserver
                ),
                info: "https://www.googleapis.com/oauth2/v3/userinfo".to_string(),
                scopes: Self::convert_to_scopes(vec!["email", "profile"])
            },
            facebook: Provider {
                client: Self::create_provider(
                    &secrets.facebook,
                    "https://www.facebook.com/dialog/oauth",
                    "https://graph.facebook.com/oauth/access_token",
                    "/api/oauth/callback/facebook",
                    &settings.webserver
                ),
                info: "https://graph.facebook.com/me?fields=name,email".to_string(),
                scopes: Self::convert_to_scopes(vec!["email", "public_profile"])
            },
            linkedin: Provider {
                client: Self::create_provider(
                    &secrets.linkedin,
                    "https://www.linkedin.com/oauth/v2/authorization",
                    "https://www.linkedin.com/oauth/v2/accessToken",
                    "/api/oauth/callback/linkedin",
                    &settings.webserver
                ),
                info: "https://api.linkedin.com/v2/userinfo".to_string(),
                scopes: Self::convert_to_scopes(vec!["profile", "email", "openid"])
            },
        }
    }

    fn create_provider(
        oauth_secret: &OauthSecret,
        auth_url: &str,
        token_url: &str,
        redirect_uri: &str,
        webserver: &WebServerSettings,
    ) -> BasicClient {
        Self::oauth_client(&oauth_secret.client_id, &oauth_secret.client_secret, auth_url, token_url)
            .set_redirect_uri(
                RedirectUrl::new(format!("{}{}", webserver.protocol_url(), redirect_uri))
                    .unwrap()
            )
    }

    fn oauth_client(
        client_id: &String,
        client_secret: &String,
        auth_url: &str,
        token_url: &str,
    ) -> BasicClient {
        BasicClient::new(
            ClientId::new(client_id.to_string()),
            Some(ClientSecret::new(client_secret.to_string())),
            AuthUrl::new(auth_url.to_string()).unwrap(),
            Some(TokenUrl::new(token_url.to_string()).unwrap()),
        )
    }

    fn convert_to_scopes(scopes: Vec<&str>) -> Vec<Scope> {
        scopes.iter().map(|s| Scope::new(s.to_string())).collect::<Vec<Scope>>()
    }

    pub fn get_provider(&self, oauth_type: &str) -> Result<&Provider, ()> {
        match oauth_type {
            "google" => Ok(&self.google),
            "facebook" => Ok(&self.facebook),
            "linkedin" => Ok(&self.linkedin),
            _ => Err(()),
        }
    }
}

impl Provider {
    pub fn auth_url(&self) -> (Url, CsrfToken) {
        self.client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(self.scopes.clone())
            .url()
    }

    pub async fn get_token(&self, code: String) -> BasicTokenResponse {
        self.client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
            .unwrap()
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