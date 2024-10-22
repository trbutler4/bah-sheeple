use anyhow;
use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RequestTokenError, Scope,
    StandardTokenResponse, TokenResponse, TokenUrl,
};
use url::Url;

pub fn create_client() -> Result<BasicClient, anyhow::Error> {
    let client = BasicClient::new(
        ClientId::new("client_id".to_string()),
        Some(ClientSecret::new("client_secret".to_string())),
        AuthUrl::new("http://authorize".to_string())?,
        Some(TokenUrl::new("http://token".to_string())?),
    )
    .set_redirect_uri(RedirectUrl::new("http://redirect".to_string())?);

    Ok(client)
}

pub fn genetate_challenge() -> (PkceCodeChallenge, PkceCodeVerifier) {
    PkceCodeChallenge::new_random_sha256()
}

pub fn generate_authorization_url(
    client: BasicClient,
    pkce_challenge: PkceCodeChallenge,
) -> (url::Url, CsrfToken) {
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("read".to_string()))
        .add_scope(Scope::new("write".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    (auth_url, csrf_token)
}

pub async fn get_access_token(
    client: BasicClient,
    pkce_verifier: PkceCodeVerifier,
) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, anyhow::Error> {
    let token_result = client
        .exchange_code(AuthorizationCode::new(
            "some authorization code".to_string(),
        ))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await?;

    Ok(token_result)
}
