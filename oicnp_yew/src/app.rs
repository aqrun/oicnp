use yew::prelude::*;
use yew_router::history::{AnyHistory, History};
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <main>
                <Switch<Route> render={Switch::render(switch)} />
            </main>
            <footer>
                footer data
            </footer>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! {
                <div>this is home</div>
            }
        }
        Route::NotFound => {
            html! {
                <div>
                  page not found
                </div>
            }
        }
    }
}