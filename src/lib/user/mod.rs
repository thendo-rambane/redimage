mod account;

use std::time::SystemTime;

pub use account::Account;
use reqwest::{blocking::Client, header::USER_AGENT};
use serde::{Deserialize, Serialize};

use crate::{
    aliases::Result,
    auth::{Auth, AuthRequest, TokenData},
    errors::{api_errors::ApiError, auth_errors::AuthError},
    response::RedditListing,
    subreddit::Subreddit,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    account_data: Option<Account>,
    signed_in_at: Option<SystemTime>,
    token_data: Option<TokenData>,
}

impl User {
    pub fn new() -> Self {
        Self {
            account_data: None,
            signed_in_at: None,
            token_data: None,
        }
    }
    pub fn authenticate(&mut self, auth_data: &AuthRequest) -> anyhow::Result<TokenData> {
        let token_data = Auth::authenticate_private(auth_data)?;
        self.signed_in_at = Some(std::time::SystemTime::now());
        self.token_data = Some(token_data.to_owned());
        Ok(token_data.clone())
    }
    pub fn user_token(&self) -> std::result::Result<TokenData, AuthError> {
        let token_data = self
            .token_data
            .to_owned()
            .ok_or(AuthError::Unathenticated("User Token Not Found".into()))?;
        Ok(token_data)
    }

    fn authenticated_request<T>(user: &User, request_url: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        if user
                .user_token_active()
                .map_err(|error| ApiError::AuthError(error))?
            {
                let client = Client::new();

                let response = client
                    .get(request_url)
                    .bearer_auth(
                    user.user_token()
                            .map_err(|error| ApiError::AuthError(error))?
                            .access_token,
                    )
                    .header(
                        USER_AGENT,
                        format!(
                            "RustClient:redimage by {}",
                            std::env::var("REDDIT_USERNAME")
                                .map_err(|error| ApiError::EnvVarError(error))?
                        ),
                    )
                    .send()?;
            let response_text = response
                        .text()
                        .map_err(|error| ApiError::TextDecodingError {
                            source: anyhow::Error::from(error),
                        })?;
            let data = serde_json::from_str::<T>(&response_text).map_err(|error| {
                        ApiError::SerdeError {
                            source: anyhow::Error::from(error),
                        }
                    })?;
            Ok(data)
            } else {
                Err(ApiError::AuthError(AuthError::Unathenticated(
                    "Authentication failed at get user token data".into(),
                )))
            }
        }

    pub fn user_token(&self) -> std::result::Result<TokenData, AuthError> {
        let token_data = self
            .token_data
            .to_owned()
            .ok_or(AuthError::Unathenticated("User Token Not Found".into()))?;
        Ok(token_data)
    }

    pub fn user_token_active(&self) -> std::result::Result<bool, AuthError> {
        let token_data = self.user_token()?;

        match self.signed_in_at {
            Some(sign_in_time) => {
                let sign_in_time = sign_in_time
                    .elapsed()
                    .map_err(|error| AuthError::TimeStampError(error))?;
                if sign_in_time.as_secs() > token_data.expires_in {
                    Err(AuthError::Unathenticated("User Token expired".into()))
                } else {
                    Ok(true)
                }
            }
            None => Err(AuthError::Unathenticated("User Token not set".into())),
        }
    }

    pub fn get_account_data(&self) -> Result<Account> {
        let request_url = "https://oauth.reddit.com/api/v1/me/?raw_json=1";
        User::authenticated_request::<Account>(self, request_url)
    }

    pub fn get_following(&self, limit: u32) -> Result<RedditListing<Subreddit>> {
            let request_url = format!(
                "https://oauth.reddit.com/subreddits/mine/?limit={limit}&raw_json=1",
                limit = limit
            );
        User::authenticated_request::<RedditListing<Subreddit>>(self, request_url.as_str())
    }

    pub fn get_account_karma_breakdown() -> Result<String> {
        todo!()
    }
}
