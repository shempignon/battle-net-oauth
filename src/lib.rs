use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
}

/// To retrieve a token, you need to provide your client_id and client_secret
///
/// ```rust
/// let token = battle_net_oauth::get_oauth_token("client_id", "client_secret");
/// ```
pub fn get_oauth_token(client_id: &str, client_secret: &str) -> Result<OAuthToken, Error> {
    let token_client = Client::new();

    let resp: OAuthToken = token_client
        .post("https://eu.battle.net/oauth/token")
        .basic_auth(client_id, Some(client_secret))
        .form(&[("grant_type", "client_credentials")])
        .send()?
        .error_for_status()?
        .json()?;

    Ok(resp)
}
