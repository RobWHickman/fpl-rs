// To generate an access token for an account need account details.
//
// These should be stored in the environment at runtime, ideally as constants
// in a .env file in the root dir.
//
// Expect two variables specific to an account:
// 1. `EMAIL`
// 2. `PASSWORD`
// See README for more.

use crate::error::FplError;
use secrecy::SecretString;
use std::env;

#[derive(Debug, Clone)]
pub struct LoginSecrets {
    email: String,
    password: SecretString,
}

impl LoginSecrets {
    pub fn from_env() -> Result<Self, FplError> {
        let email: String =
            env::var("EMAIL").map_err(|_| FplError::MissingEnvVar("EMAIL".to_string()))?;

        let password =
            env::var("PASSWORD").map_err(|_| FplError::MissingEnvVar("PASSWORD".to_string()))?;

        Ok(LoginSecrets {
            email,
            password: SecretString::new(password.into_boxed_str()),
        })
    }

    #[must_use]
    pub fn email(&self) -> &str {
        &self.email
    }

    #[must_use]
    pub fn password(&self) -> &SecretString {
        &self.password
    }
}
