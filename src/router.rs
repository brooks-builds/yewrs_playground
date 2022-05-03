use crate::pages::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Debug, PartialEq, Clone, Copy)]
pub enum Route {
    #[at("/")]
    Home,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
    }
}
