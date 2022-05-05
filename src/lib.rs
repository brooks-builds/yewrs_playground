mod context;
pub mod pages;
mod router;
pub mod utils;
mod yewdux_store;

use context::AppContext;
use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let context = use_state(AppContext::default);

    let mut auth0 = context.auth0.clone();
    auth0.handleRedirectCallback().unwrap();

    html! {
      <div>
        <ContextProvider<AppContext> context={(*context).clone()}>
          <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
            <ul>
              <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
              <li><Link<Route> to={Route::UsingAComponentLibrary}>{"Using a Component Library"}</Link<Route>></li>
              <li><Link<Route> to={Route::Authentication}>{"Authentication"}</Link<Route>></li>
            </ul>
          </BrowserRouter>
        </ContextProvider<AppContext>>
      </div>
    }
}
