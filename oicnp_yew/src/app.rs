use yew::prelude::*;
use yew_router::history::{AnyHistory, History};
use yew_router::prelude::*;
use crate::routes::{switch, AppRoute};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <main class="h-full">
                <Switch<AppRoute> render={Switch::render(switch)} />
            </main>
        </BrowserRouter>
    }
}
