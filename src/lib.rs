use reqwest::{Client, Error};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
}

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
