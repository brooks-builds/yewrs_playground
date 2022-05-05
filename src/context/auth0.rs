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
    pub fn handleRedirectCallback(&mut self) -> Result<()> {
        gloo::console::log!("handling Redirect Callback");
        self.loading = true;
        let url = Url::parse(&self.get_url()?)?;
        let authentication_result = AuthenticationResult::new(url);
        gloo::console::log!(
            "code:",
            authentication_result.code,
            "state",
            authentication_result.state
        );
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

#[derive(Default)]
struct AuthenticationResult {
    pub state: String,
    pub code: String,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

impl AuthenticationResult {
    pub fn new(url: Url) -> Self {
        gloo::console::log!(url.query().unwrap());
        let mut authentication_result = Self::default();
        for (key, value) in url.query_pairs() {
            gloo::console::log!("key:", &*key, "value:", &*value);
            match &*key {
                "state" => authentication_result.state = (*value).to_owned(),
                "code" => authentication_result.code = (*value).to_owned(),
                "error" => authentication_result.error = Some((*value).to_owned()),
                "error_description" => authentication_result.error = Some((*value).to_owned()),
                _ => (),
            }
        }
        authentication_result
    }
}
