use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::{switch, AppRoute};

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={switch} />
        </BrowserRouter>
    }
}

