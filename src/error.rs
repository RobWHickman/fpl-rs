use thiserror::Error;

#[derive(Error, Debug)]
pub enum FplError {
    #[error("Access token not found in HTML response")]
    AccessTokenNotFound,

    #[error("State not found in HTML response")]
    StateNotFound,

    #[error("Authorization code not found in redirect URL")]
    AuthCodeNotFound,

    #[error("Invalid regex pattern: {0}")]
    RegexError(#[from] regex::Error),

    #[error("Invalid request: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Invalid header value: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),

    #[error("Missing required header: {0}")]
    MissingHeader(String),

    #[error("Invalid header value encoding: {0}")]
    InvalidHeaderEncoding(#[from] reqwest::header::ToStrError),
    
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("Field not found in JSON return: {0}")]
    JsonField(String),
}
