use anyhow::Result;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenData {
    pub access_token: String,
    pub expires_in: u64,
    scope: String,
    token_type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthRequest {
    // grant_type: String,
    pub username: String,
    pub password: String,
    pub client_id: String,
    pub client_secret: Option<String>,
}

pub struct Auth {}

impl Auth {
    // pub fn authenticate_oauth(_auth_data: &AuthRequest) -> Result<TokenData> {
    //     todo!()
    // }
    pub fn authenticate_private(auth_data: &AuthRequest) -> Result<TokenData> {
        //TODO {Thendo}: Create Error Types
        let client = Client::new();
        let request_url = format!("https://www.reddit.com/api/v1/access_token");
        let response = client
            .post(request_url)
            .basic_auth(
                auth_data.client_id.as_str(),
                // "asdasdsad",
                auth_data.client_secret.as_ref(),
            )
            .body(format!(
                "grant_type=password&username={}&password={}",
                auth_data.username.as_str(),
                auth_data.password.as_str()
            ))
            .send()?;
        let res_text = response.text()?;
        let response = serde_json::from_str::<TokenData>(&res_text)?;
        // let now = std::time::SystemTime::now();
        Ok(response)
    }
}
