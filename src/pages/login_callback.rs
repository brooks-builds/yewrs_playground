use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};
use yewdux::prelude::Dispatcher;
use yewdux_functional::use_store;

use crate::{
    auth::auth0::{Auth0, AuthenticationResult},
    router::Route,
    yewdux_store::AuthStoreType,
};

#[function_component(LoginCallback)]
pub fn login_callback() -> Html {
    let dispatch = use_store::<AuthStoreType>().dispatch().clone();
    let (loading, is_authenticated) = use_store::<AuthStoreType>()
        .state()
        .map(|s| (s.loading, s.is_authenticated))
        .unwrap_or_default();
    let history = use_history().unwrap();
    if !loading && !is_authenticated {
        dispatch.clone().reduce(move |store| {
            store.loading = true;
        });

        let raw_url = Auth0::get_url().unwrap();
        let authentication = AuthenticationResult::new(raw_url).unwrap();
        let stored_state = Auth0::get_state().unwrap();
        if authentication.state == stored_state {
            dispatch
                .clone()
                .reduce(|store| store.is_authenticated = true);
        }

        {
            let dispatch = dispatch.clone();
            let history = history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let user = Auth0::get_user(&authentication.access_token)
                    .await
                    .unwrap()
                    .clone();
                {
                    let user = user.clone();
                    dispatch.reduce(move |s| {
                        s.user = Some(user);
                    });
                }
                gloo::console::log!("Username, ", &user.name);
                history.push(Route::Authentication);
            });
        }
        dispatch.reduce(|s| s.loading = false);
    }
    html! {
      <div></div>
    }
}
