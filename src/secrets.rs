// To generate an access token for an account need account details.
// 
// These should be stored in the environment at runtime, ideally as constants
// in a .env file in the root dir.
//
// Expect two variables specific to an account:
// 1. `EMAIL`
// 2. `PASSWORD`
// See README for more.

use secrecy::{SecretString};
use std::env;

#[derive(Debug, Clone)]
pub struct LoginSecrets {
    email: String,
    password: SecretString,
}

impl LoginSecrets {
    pub fn from_env() -> Result<Self, String> {
        let email = env::var("EMAIL")
            .map_err(|_| "EMAIL environment variable not set".to_string())?;
        let password = env::var("PASSWORD")
            .map_err(|_| "PASSWORD environment variable not set".to_string())?;

        Ok(LoginSecrets {
            email,
            password: SecretString::new(password.into_boxed_str()),
        })
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &SecretString {  // Changed
        &self.password
    }
}