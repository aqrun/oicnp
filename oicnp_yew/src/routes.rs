use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{Home, NotFound, Login};

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &AppRoute) -> Html {
    match route {
        AppRoute::Login => html! { <Login /> },
        AppRoute::Home => html! { <Home /> },
        _ => html! { <NotFound /> },
    }
}