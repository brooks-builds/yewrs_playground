use std::env;

use rand_os::rand_core::RngCore;
use yew::prelude::*;
use yew_router::{
    history::{self, Location},
    hooks::{use_history, use_location},
};
use yewdux_functional::use_store;

use crate::{
    utils::{create_random_string, log_string},
    yewdux_store::AuthStoreType,
};

#[function_component(Authentication)]
pub fn authentication() -> Html {
    let (auth_loading, is_authenticated) = use_store::<AuthStoreType>()
        .state()
        .map(|s| (s.loading, s.is_authenticated))
        .unwrap_or_default();

    let login_onclick = Callback::from(|_event: MouseEvent| {
        let login_url = create_login_url();
        log_string(&login_url);
        gloo::utils::window()
            .location()
            .set_href(&login_url)
            .unwrap();
    });

    let logout_onclick = Callback::from(move |_event: MouseEvent| {
        let logout_url = create_logout_url("http://localhost:8080/authentication");
        // this can be replaced with a use_navigator after yew_router 0.17 or later is released
        gloo::utils::window()
            .location()
            .set_href(&logout_url)
            .unwrap();
    });

    let location = use_location().unwrap();
    log_string(&location.search().to_string());

    html! {
      <div>
        <h1>{"Authentication"}</h1>
            <button onclick={logout_onclick}>{"Logout"}</button>
            <button onclick={login_onclick}>{"login"}</button>
      </div>
    }
}

fn create_login_url() -> String {
    let domain = env!("AUTH0_DOMAIN");
    let client_id = env!("AUTH0_CLIENT_ID");
    let redirect_uri = "http://localhost:8080/authentication";
    let state = create_random_string();

    format!("https://{domain}/authorize?response_type=token&client_id={client_id}&redirect_uri={redirect_uri}&scope=openid%20profile%20email&state={state}")
}

fn create_logout_url(return_to: &str) -> String {
    let domain = env!("AUTH0_DOMAIN");
    let client_id = env!("AUTH0_CLIENT_ID");
    format!("https://{domain}/v2/logout?client_id={client_id}&returnTo={return_to}")
}
