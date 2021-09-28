use dotenv::dotenv;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct AuthResponse {
    access_token: String,
    expires_in: i32,
    scope: String,
    token_type: String,
}

#[derive(Serialize, Deserialize)]
struct AuthRequestData {
    grant_type: String,
    username: String,
    password: Option<String>,
}

fn authenticate() -> Result<AuthResponse> {
    //TODO {Thendo}: Handle Errors properly
    let username = std::env::var("REDDIT_USERNAME").unwrap();
    let password = std::env::var("REDDIT_PASSWORD").unwrap();
    let client_id = std::env::var("REDDIT_CLIENT_ID").unwrap();
    let client_secret = std::env::var("REDDIT_CLIENT_SECRET").ok();

    let client = Client::new();
    let request_url = format!("https://www.reddit.com/api/v1/access_token");
    let response = client
        .post(request_url)
        .basic_auth(client_id, client_secret)
        .body(format!(
            "grant_type=password&username={}&password={}",
            username.as_str(),
            password.as_str()
        ))
        .send()
        .unwrap()
        .text()
        .unwrap();
    serde_json::from_str::<AuthResponse>(&response)
}

fn main() {
    dotenv().unwrap();
    authenticate().unwrap();
}
