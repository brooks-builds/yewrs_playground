use yew::prelude::*;
use yew_component_library::atoms::bb_text::{text_type::BBTextType, BBText};

#[function_component(UsingAComponentLibrary)]
pub fn using_a_component_library() -> Html {
    html! {
      <div>
        <BBText text_type={BBTextType::Title} text="Yew.rs Playground" />
        <BBText text="This is a paragraph" />
      </div>
    }
}
