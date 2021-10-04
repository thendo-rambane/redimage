use thiserror::Error;

use super::auth_errors;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Failed to decode response text")]
    TextDecodingError {
        // #[source]
        source: anyhow::Error,
    },

    #[error("Failed to parse json for")]
    SerdeError { source: anyhow::Error },

    #[error("The request library threw an error")]
    RequestError(#[from] reqwest::Error),

    #[error(transparent)]
    AuthError(#[from] auth_errors::AuthError),

    #[error("Could not read environment variable {0}")]
    EnvVarError(std::env::VarError),
}
