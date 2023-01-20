use yew::prelude::*;
use crate::components::Header;

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct MainLayoutProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

#[function_component]
pub fn MainLayout(props: &MainLayoutProps) -> Html {
    html! {
        <div class={classes!(
            "main-layout-container",
            "min-h-screen",
            &props.classes
        )} >
            <header
                class="main-layout-header shadow-lg fixed top-0
                    m-0 min-w-full bg-gray-900 h-16"
            >
                <Header />
            </header>
            <aside class="main-layout-side main-height shadow-md
                fixed top-16 w-80
                left-0 overflow-y-auto border-r border-solid
                border-gray-200"
            >
                {"nav"}
            </aside>
            <main
                class="main-layout-content main-height bg-gray-100
                    overflow-y-auto m-0 mt-16 ml-80"
            >
                <section class="main-layout-page min-h-screen">
                    {props.children.clone()}
                </section>
                <footer
                    class="main-layout-footer h-16 bg-gray-300
                        text-center text-white"
                >
                    {"footer"}
                </footer>
            </main>
        </div>
    }
}

