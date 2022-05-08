mod auth;
pub mod pages;
mod router;
pub mod utils;
mod yewdux_store;

use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    gloo::console::log!("App");

    html! {
      <div>
          <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
            <ul>
              <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
              <li><Link<Route> to={Route::UsingAComponentLibrary}>{"Using a Component Library"}</Link<Route>></li>
              <li><Link<Route> to={Route::Authentication}>{"Authentication"}</Link<Route>></li>
            </ul>
          </BrowserRouter>
      </div>
    }
}
