use thiserror::Error;

use super::auth_errors;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Failed to decode {data_type} from response text")]
    TextDecodingError {
        // #[source]
        source: anyhow::Error,
        data_type: String,
    },

    #[error("Failed to parse json for {data_type}")]
    SerdeError {
        source: anyhow::Error,
        data_type: String,
    },

    #[error("The request library threw an error")]
    RequestError(#[from] reqwest::Error),

    #[error(transparent)]
    AuthError(#[from] auth_errors::AuthError),

    #[error("Could not read environment variable {0}")]
    EnvVarError(std::env::VarError),
}
