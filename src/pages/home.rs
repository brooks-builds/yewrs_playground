use yew::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::use_store;

use crate::yewdux_store::AuthStore;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
      <div>
        <h1>{"Home"}</h1>
      </div>
    }
}
