mod aliases;
use aliases::Result;

mod response;
use response::RedditListing;

mod auth;
use auth::{AuthRequest, TokenData};

mod errors;

mod subreddit;
use subreddit::Subreddit;

mod user;
use user::{Account, User};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditApi {
    auth_data: auth::AuthRequest,
    user: User,
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
    pub fn get_account(&mut self) -> Result<Account> {
        self.user.get_account_data()
    }
    pub fn get_following(&self, limit: u32) -> Result<RedditListing<Subreddit>> {
        self.user.get_following(limit)
    }
}
