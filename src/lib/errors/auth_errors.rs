use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Unathenticated: \n\t{0}.\n\tPlease Authenticate")]
    Unathenticated(String),

    #[error(transparent)]
    TimeStampError(#[from] std::time::SystemTimeError),
}
