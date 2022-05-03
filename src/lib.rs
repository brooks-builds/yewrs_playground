pub mod pages;
mod router;
mod yewdux_store;

use router::{switch, Route};
use yew::prelude::*;
use yew_component_library::atoms::bb_text::{text_type::BBTextType, BBText};
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
      <div>
        <BrowserRouter>
          <Switch<Route> render={Switch::render(switch)} />
          <BBText text_type={BBTextType::Title} text="Yew.rs Playground" />
          <BBText text="This is a paragraph" />
        </BrowserRouter>
      </div>
    }
}
