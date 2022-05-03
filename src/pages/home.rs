use yew::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::use_store;

use crate::yewdux_store::AuthStore;

#[function_component(Home)]
pub fn home() -> Html {
    let (auth_loading, is_authenticated) = use_store::<BasicStore<AuthStore>>()
        .state()
        .map(|s| (s.loading, s.is_authenticated))
        .unwrap_or_default();
    html! {
      <div>
        <h1>{"Home"}</h1>
        if !auth_loading {
          if is_authenticated {
            <button>{"Logout"}</button>
          } else {
            <button>{"login"}</button>
          }
        }
      </div>
    }
}
