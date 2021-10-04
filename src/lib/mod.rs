mod aliases;
mod auth;
mod errors;
mod response;
mod subreddit;
mod user;

use auth::{AuthRequest, TokenData};
use serde::{Deserialize, Serialize};
use user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditApi {
    auth_data: auth::AuthRequest,
    pub user: User,
}

impl RedditApi {
    pub fn new() -> Self {
        RedditApi {
            auth_data: AuthRequest {
                username: std::env::var("REDDIT_USERNAME").unwrap(),
                password: std::env::var("REDDIT_PASSWORD").unwrap(),
                client_id: std::env::var("REDDIT_CLIENT_ID").unwrap(),
                client_secret: std::env::var("REDDIT_CLIENT_SECRET").ok(),
            },
            user: User::new(),
        }
    }
    pub fn authenticate(&mut self) -> anyhow::Result<TokenData> {
        self.user.authenticate(&self.auth_data)
    }
}
