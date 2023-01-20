mod page_title;

use yew::prelude::*;
use crate::components::{
    BreadItem, Breadcrumb,
};
pub use page_title::*;

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct PageHeaderProps {
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub bread_items: Vec<BreadItem>,
}

#[function_component]
pub fn PageHeader(props: &PageHeaderProps) -> Html {
    html! {
        <section
            class="page-header-container p-8 bg-white"
        >
            {if props.bread_items.len() > 0 {
                html! { <Breadcrumb items={props.bread_items.clone()} /> }
            } else {
                html! {}
            }}
            {if !props.title.is_empty() {
                html! {
                    <PageTitle title={props.title.clone()} />
                }
            } else {
                html! {}
            }}
        </section>
    }
}
