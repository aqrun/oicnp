use yew::prelude::*;

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct NavItemProps {
    #[prop_or_default]
    pub label: String,
}

#[function_component]
pub fn NavItem(props: &NavItemProps) -> Html {
    html! {
        <a
            class="navbar-item px-2 h-16 hover:bg-gray-600 flex items-center
                cursor-pointer text-xl"
        >
            { props.label.clone() }
        </a>
    }
}

