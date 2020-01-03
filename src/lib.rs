use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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
pub async fn get_oauth_token(
    client_id: &str,
    client_secret: &str,
) -> Result<OAuthToken, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let resp: OAuthToken = client
        .post("https://eu.battle.net/oauth/token")
        .basic_auth(client_id, Some(client_secret))
        .form(&[("grant_type", "client_credentials")])
        .send()
        .await?
        .json()
        .await?;

    Ok(resp)
}
