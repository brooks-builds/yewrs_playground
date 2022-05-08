use crate::pages::{
    authentication::Authentication, home::Home, login_callback::LoginCallback,
    using_a_component_library::UsingAComponentLibrary,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Debug, PartialEq, Clone, Copy)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/using-a-component-library")]
    UsingAComponentLibrary,
    #[at("/authentication")]
    Authentication,
    #[at("login-callback")]
    LoginCallback,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::UsingAComponentLibrary => html! { <UsingAComponentLibrary /> },
        Route::Authentication => html! { <Authentication /> },
        Route::LoginCallback => html! { <LoginCallback /> },
    }
}
