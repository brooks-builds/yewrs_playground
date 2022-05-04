use std::env;

use rand_os::rand_core::RngCore;
use yew::prelude::*;
use yewdux_functional::use_store;

use crate::{utils::log_string, yewdux_store::AuthStoreType};

#[function_component(Authentication)]
pub fn authentication() -> Html {
    let (auth_loading, is_authenticated) = use_store::<AuthStoreType>()
        .state()
        .map(|s| (s.loading, s.is_authenticated))
        .unwrap_or_default();

    let login_onclick = Callback::from(|_event: MouseEvent| {
        let login_url = create_login_url();
        log_string(&login_url);
        gloo::utils::window().location().set_href(&login_url);
    });

    html! {
      <div>
        <h1>{"Authentication"}</h1>
        if !auth_loading {
          if is_authenticated {
            <button>{"Logout"}</button>
          } else {
            <button onclick={login_onclick}>{"login"}</button>
          }
        }
      </div>
    }
}

fn create_login_url() -> String {
    let domain = env!("AUTH0_DOMAIN");
    let client_id = env!("AUTH0_CLIENT_ID");
    let redirect_uri = "http://localhost:8080/authentication";
    let mut random_numbers = [0u8; 16];

    rand_os::rand_core::OsRng.fill_bytes(&mut random_numbers);
    let mut state = random_numbers
        .into_iter()
        .map(|item: u8| item.to_string())
        .collect::<Vec<String>>()
        .join("");

    format!("https://{domain}/authorize?response_type=token&client_id={client_id}&redirect_uri={redirect_uri}&scope=openid%20profile%20email&state={state}")
}
