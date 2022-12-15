use yew::prelude::*;

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct BreadLinkItemProps {
    #[prop_or_default]
    pub label: String,
}

#[function_component]
pub fn BreadLinkItem(props: &BreadLinkItemProps) -> Html {
    html! {
        <a class="px-2 cursor-pointer text-gray-400 hover:text-gray-700">
            { props.label.clone() }
        </a>
    }
}
