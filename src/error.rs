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

    #[error("Field not found in JSON return: {0}")]
    JsonField(String),
}
