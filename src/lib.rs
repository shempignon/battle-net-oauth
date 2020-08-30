use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
}

/// To retrieve a token, you need to provide your client_id and client_secret as well as a region (US, EU, APAC or CN)
///
/// ```rust
/// let token = battle_net_oauth::get_oauth_token("client_id", "client_secret", "region");
/// ```
pub async fn get_oauth_token(
    client_id: &str,
    client_secret: &str,
    region: &str,
) -> Result<OAuthToken, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let url = if region.to_lowercase() == "cn" {
        "https://www.battlenet.com.cn/oauth/token".to_string()
    } else {
        format!("https://{}.battle.net/oauth/token", region.to_lowercase())
    };

    let resp: OAuthToken = client
        .post(&url)
        .basic_auth(client_id, Some(client_secret))
        .form(&[("grant_type", "client_credentials")])
        .send()
        .await?
        .json()
        .await?;

    Ok(resp)
}
