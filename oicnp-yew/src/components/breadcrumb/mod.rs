mod bread_link_item;

use yew::prelude::*;
use yew::Html;
use self::bread_link_item::BreadLinkItem;

#[derive(PartialEq, Debug, Clone)]
pub struct BreadItem {
    pub label: String,
}

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct BreadcrumbProps {
    #[prop_or_default]
    pub items: Vec<BreadItem>,
}

#[function_component]
pub fn Breadcrumb(props: &BreadcrumbProps) -> Html {

    if props.items.len() == 0 {
        return html!{};
    }

    html! {
        <nav class="breadcrumb">
            <ul class="flex items-center">
                {props.items.clone().into_iter().map(|item| {
                    html! {
                        <li class="breadcrumb-li">
                            <BreadLinkItem
                                label={item.label}
                            />
                        </li>
                    }
                }).collect::<Html>()}
            </ul>
        </nav>
    }
}
