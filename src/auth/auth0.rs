use super::user::User;
use eyre::{bail, Result};
use js_sys::global;
use rand_os::rand_core::RngCore;
use serde::{Deserialize, Serialize};
use url::Url;
use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;

#[derive(Clone, PartialEq, Default)]
pub struct Auth0 {
    pub loading: bool,
    pub is_authenticated: bool,
    pub user: Option<User>,
}

impl Auth0 {
    pub fn get_url() -> Result<String> {
        match gloo::utils::window().location().href() {
            Ok(url) => Ok(url),
            Err(error) => {
                gloo::console::error!(error);
                bail!("Error getting url")
            }
        }
    }

    pub fn handle_login() {
        let state = Auth0::create_random_string();
        let login_url = Auth0::create_login_url(&state);
        Auth0::store_state(&state);
        gloo::utils::window()
            .location()
            .set_href(&login_url)
            .unwrap();
    }

    pub fn create_random_string() -> String {
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

    fn create_login_url(state: &str) -> String {
        let domain = env!("AUTH0_DOMAIN");
        let client_id = env!("AUTH0_CLIENT_ID");
        let redirect_uri = "http://localhost:8080/login-callback";

        format!("https://{domain}/authorize?response_type=token&client_id={client_id}&redirect_uri={redirect_uri}&scope=profile openid email&state={state}")
    }

    fn store_state(state: &str) {
        let document = gloo::utils::document();
        let html_document = document.dyn_into::<HtmlDocument>().unwrap();
        let cookie = format!("auth0state={state}; SameSite=Strict; Secure");
        html_document.set_cookie(&cookie);
    }

    pub fn get_state() -> Result<String> {
        let cookie_string = gloo::utils::document()
            .dyn_into::<HtmlDocument>()
            .unwrap()
            .cookie()
            .unwrap();
        let mut raw_cookies = cookie_string.split("; ");
        // raw_cookies.find(|cookie| {
        //     if cookie.contains("auth0state=") {
        //         if let Some(cookie_state) = cookie.split('=').last() {
        //             cookie_state.to_owned()
        //         }
        //     }
        //     gloo::console::log!(*item);
        //     false
        // });
        for raw_cookie in raw_cookies {
            if !raw_cookie.contains("auth0state=") {
                continue;
            }

            return Ok(raw_cookie.split('=').last().unwrap().to_owned());
        }

        bail!("could not find state cookie");
    }

    pub async fn get_user(token: &str) -> Result<User> {
        let domain = env!("AUTH0_DOMAIN");
        let url = format!("https://{domain}/userinfo");
        let bearer_token = format!("Bearer {token}");
        let response = gloo::net::http::Request::get(&url)
            .header("Authorization", &bearer_token)
            .send()
            .await?;

        if !response.ok() {
            gloo::console::error!(response.status_text());
            bail!("error in response");
        }

        Ok(response.json::<User>().await?)
    }

    pub fn handle_logout() {
        let domain = env!("AUTH0_DOMAIN");
        let client_id = env!("AUTH0_CLIENT_ID");
        let logout_url = env!("LOGOUT_URL");
        gloo::utils::window()
            .location()
            .set_href(&format!(
                "https://{domain}/v2/logout?client_id={client_id}&returnTo={logout_url}"
            ))
            .unwrap();
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct AuthenticationResult {
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
