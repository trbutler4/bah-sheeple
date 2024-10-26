use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub app_base_url: String,
    pub auth_url: String,
    pub token_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        Self {
            client_id: std::env::var("CLIENT_ID").expect("CLIENT_ID must be set"),
            client_secret: std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set"),
            app_base_url: "https://proud-maggot-initially.ngrok-free.app".to_string(),
            auth_url: "https://twitter.com/i/oauth2/authorize".to_string(),
            token_url: "https://api.twitter.com/2/oauth2/token".to_string(),
        }
    }

    pub fn create_oauth_client(&self) -> BasicClient {
        let redirect_url = format!("{}/oauth/callback", self.app_base_url);

        BasicClient::new(
            ClientId::new(self.client_id.clone()),
            Some(ClientSecret::new(self.client_secret.clone())),
            AuthUrl::new(self.auth_url.clone()).unwrap(),
            Some(TokenUrl::new(self.token_url.clone()).unwrap()),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
    }
}
