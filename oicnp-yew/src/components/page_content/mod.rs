use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct PageContentProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn PageContent(props: &PageContentProps) -> Html {
    html! {
        <section class="page-content bg-white m-8">
            { props.children.clone() }
        </section>
    }
}
