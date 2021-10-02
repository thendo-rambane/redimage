mod account;

use std::time::SystemTime;

pub use account::Account;
use anyhow::{bail, Result};
use reqwest::{blocking::Client, header::USER_AGENT};
use serde::{Deserialize, Serialize};

use crate::{
    auth::{Auth, AuthRequest, TokenData},
    errors::{api_request_errors::ApiRequestError, auth_errors::AuthError},
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
    pub fn authenticate(&mut self, auth_data: &AuthRequest) -> Result<TokenData> {
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

    pub fn get_account_data(&mut self) -> std::result::Result<Account, ApiRequestError> {
        if let Some(account_data) = self.account_data.clone() {
            Ok(account_data.clone())
        } else {
            if self
                .user_token_active()
                .map_err(|error| ApiRequestError::AuthError(error))?
            {
                let client = Client::new();
                let request_url = format!("https://oauth.reddit.com/api/v1/me/?raw_json=1");

                let response = client
                    .get(request_url)
                    .bearer_auth(
                        self.user_token()
                            .map_err(|error| ApiRequestError::AuthError(error))?
                            .access_token,
                    )
                    .header(
                        USER_AGENT,
                        format!(
                            "RustClient:redimage by {}",
                            std::env::var("REDDIT_USERNAME")
                                .map_err(|error| ApiRequestError::EnvVarError(error))?
                        ),
                    )
                    .send()?;
                let response_text =
                    response
                        .text()
                        .map_err(|error| ApiRequestError::TextDecodingError {
                            source: anyhow::Error::from(error),
                            data_type: "Account".into(),
                        })?;
                let user_account =
                    serde_json::from_str::<Account>(&response_text).map_err(|error| {
                        ApiRequestError::SerdeError {
                            data_type: "Account".into(),
                            source: anyhow::Error::from(error),
                        }
                    })?;
                self.account_data = Some(user_account.to_owned());
                Ok(user_account)
            } else {
                Err(ApiRequestError::AuthError(AuthError::Unathenticated(
                    "Authentication failed at get_account_data".into(),
                )))
            }
        }
    }

    pub fn get_following(&self, limit: u32) -> Result<RedditListing<Subreddit>> {
        if let Some(token_data) = self.token_data.clone() {
            if let Some(signed_in_at) = self.signed_in_at {
                if signed_in_at.elapsed()?.as_secs() > token_data.expires_in {
                    bail!("error:{message: 'unauthenticated'}")
                } else {
                    let client = Client::new();
                    let request_url = format!(
                        "https://oauth.reddit.com/subreddits/mine/?limit={limit}&raw_json=1",
                        limit = limit // username = "niasrevenge"
                                      // username = self.username
                    );
                    let response = client
                        .get(request_url)
                        .bearer_auth(token_data.access_token)
                        .header(
                            USER_AGENT,
                            format!(
                                "RustClient:redimage by {}",
                                std::env::var("REDDIT_USERNAME")?
                            ),
                        )
                        .send()?;
                    let response_text = response.text()?;
                    let subbredits_followed =
                        serde_json::from_str::<RedditListing<Subreddit>>(&response_text)?;
                    Ok(subbredits_followed)
                }
            } else {
                bail!("error:{message: 'unauthenticated'}")
            }
        } else {
            bail!("error:{message: 'unauthenticated'}")
        }
    }

    pub fn get_account_karma_breakdown() -> Result<String> {
        todo!()
    }
}
