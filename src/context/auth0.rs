use std::error::Error;

use super::user::User;
use eyre::{bail, Result};
use rand_os::rand_core::RngCore;
use serde::{Deserialize, Serialize};
use url::Url;
use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;
use yew::Html;

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
        let cookies = self.get_state();
        gloo::console::log!(cookies);
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

    pub fn handle_login(&self) {
        let state = self.create_random_string();
        let login_url = self.create_login_url(&state);
        self.store_state(&state);
        gloo::utils::window()
            .location()
            .set_href(&login_url)
            .unwrap();
    }

    pub fn create_random_string(&self) -> String {
        let mut random_numbers = [0u8; 43];
        let charset = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_~.";
        let charset_length = charset.len();

        rand_os::rand_core::OsRng.fill_bytes(&mut random_numbers);

        random_numbers
            .into_iter()
            .map(|random_number: u8| {
                let (_index, random_char) = charset
                    .char_indices()
                    .nth(random_number as usize % charset_length)
                    .unwrap();
                random_char.to_string()
            })
            .collect::<Vec<String>>()
            .join("")
    }

    fn create_login_url(&self, state: &str) -> String {
        let domain = env!("AUTH0_DOMAIN");
        let client_id = env!("AUTH0_CLIENT_ID");
        let redirect_uri = "http://localhost:8080/authentication";

        format!("https://{domain}/authorize?response_type=token&client_id={client_id}&redirect_uri={redirect_uri}&scope=openid%20profile%20email&state={state}")
    }

    fn store_state(&self, state: &str) {
        let document = gloo::utils::document();
        let html_document = document.dyn_into::<HtmlDocument>().unwrap();
        let cookie = format!("auth0state={state}; SameSite=Strict; Secure");
        html_document.set_cookie(&cookie);
    }

    fn get_state(&self) -> String {
        gloo::utils::document()
            .dyn_into::<HtmlDocument>()
            .unwrap()
            .cookie()
            .unwrap()
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
