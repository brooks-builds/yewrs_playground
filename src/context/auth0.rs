use std::error::Error;

use eyre::{bail, Result};
use serde::{Deserialize, Serialize};
use url::Url;

use super::user::User;

#[derive(Clone, PartialEq)]
pub struct Auth0 {
    pub loading: bool,
    pub is_authenticated: bool,
    pub user: User,
}

impl Auth0 {
    pub fn handle_redirect_callback(&mut self) -> Result<()> {
        self.loading = true;
        let gloo_url = self.get_url()?;
        let authentication_result = AuthenticationResult::new(gloo_url)?;
        // check the state to make sure it's the one we sent
        Ok(())
    }

    fn get_url(&self) -> Result<String> {
        match gloo::utils::window().location().href() {
            Ok(url) => Ok(url),
            Err(error) => {
                gloo::console::error!(error);
                bail!("Error getting url")
            }
        }
    }
}

impl Default for Auth0 {
    fn default() -> Self {
        Self {
            loading: true,
            is_authenticated: Default::default(),
            user: Default::default(),
        }
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
struct AuthenticationResult {
    pub access_token: String,
    pub scope: String,
    pub expires_in: i32,
    pub token_type: String,
    pub state: String,
}

impl AuthenticationResult {
    pub fn new(raw_url: String) -> Result<Self> {
        let mut authentication_result = Self::default();
        let url = Url::parse(&raw_url)?;
        let fragment = match url.fragment() {
            Some(fragment) => fragment,
            None => bail!("no fragment found in URL"),
        };
        gloo::console::log!("fragment string", fragment);
        let mut fragments = fragment.split(|character| character == '=' || character == '&');
        loop {
            let key = if let Some(key) = fragments.next() {
                key
            } else {
                break;
            };
            let value = if let Some(value) = fragments.next() {
                value.to_owned()
            } else {
                break;
            };
            match key {
                "access_token" => authentication_result.access_token = value,
                "scope" => authentication_result.scope = value,
                "expires_in" => authentication_result.expires_in = value.parse()?,
                "token_type" => authentication_result.token_type = value,
                "state" => authentication_result.state = value,
                _ => gloo::console::log!("unknown key: ", key),
            };
        }

        Ok(authentication_result)
    }
}
