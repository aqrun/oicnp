use yew::prelude::*;

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct PageTitleProps {
    #[prop_or_default]
    pub title: String,
}

#[function_component(PageTitle)]
pub fn page_title(props: &PageTitleProps) -> Html {
    html! {
        <section
            class="page-title-container mt-2.5"
        >
            <h1 class="page-title text-black text-3xl text-bold">
                { props.title.clone() }
            </h1>
        </section>
    }
}
