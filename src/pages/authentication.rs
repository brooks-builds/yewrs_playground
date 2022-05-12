use std::env;

use rand_os::rand_core::RngCore;
use yew::prelude::*;
use yew_router::{
    history::{self, Location},
    hooks::{use_history, use_location},
};
use yewdux_functional::use_store;

use crate::{
    app,
    auth::auth0::Auth0,
    utils::{create_random_string, log_string},
    yewdux_store::AuthStoreType,
};

#[function_component(Authentication)]
pub fn authentication() -> Html {
    let (auth_loading, is_authenticated, user) = use_store::<AuthStoreType>()
        .state()
        .map(|state| (state.loading, state.is_authenticated, state.user.clone()))
        .unwrap_or_default();
    let login_onclick = Callback::from(move |_event: MouseEvent| {
        Auth0::handle_login();
    });

    let logout_onclick = Callback::from(move |_event: MouseEvent| {
        Auth0::handle_logout();
    });

    html! {
      <div>
        <h1>{"Authentication"}</h1>
            if let Some(user) = user {
                <div>
                    {user.name}
                </div>
            }
            if !auth_loading {
                if is_authenticated {
                    <button onclick={logout_onclick}>{"logout"}</button>
                } else {
                    <button onclick={login_onclick}>{"login"}</button>
                }
            }
      </div>
    }
}
