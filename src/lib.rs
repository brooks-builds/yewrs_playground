use yew::prelude::*;
use yew_component_library::atoms::bb_text::{text_type::BBTextType, BBText};

#[function_component(App)]
pub fn app() -> Html {
    html! {
      <div>
        <BBText text_type={BBTextType::Title} text="Yew.rs Playground" />
        <BBText text="This is a paragraph" />
      </div>
    }
}
