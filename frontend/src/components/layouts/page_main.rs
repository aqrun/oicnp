use dioxus::prelude::*;

#[derive(Props)]
pub struct PageMainProps<'a> {
    pub class: Option<&'a str>,
    pub children: Element<'a>,
}

/// 页面主体
pub fn PageMain<'a>(cx: Scope<'a, PageMainProps<'a>>) -> Element {
    let container_class = &cx.props.class.unwrap_or("");

    cx.render(rsx!(
        main {
            class: "oic-page-main mx-auto grid {container_class}",
            &cx.props.children
        }
    ))
}