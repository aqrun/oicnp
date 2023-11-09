use dioxus::prelude::*;
use super::super::widgets::{
    BloodRecommend,
    RandomRecommends,
    HotTags,
    SiteInfo,
};

#[derive(Props)]
pub struct MainSectionProps<'a> {
    pub class: Option<&'a str>,
    pub content_class: Option<&'a str>,
    pub children: Element<'a>,
}

/// 页面主内容区布局
pub fn MainSection<'a>(cx: Scope<'a, MainSectionProps<'a>>) -> Element {
    let container_cls = &cx.props.class.unwrap_or("");
    let content_cls = &cx.props.content_class.unwrap_or("");

    cx.render(rsx!(
        section {
            class: "oic-main-section mt-6 {container_cls}",
            div {
                class: "oic-main-section-inner lg:mx-auto max-w-7xl
                    flex flex-col lg:flex-row",
                div {
                    class: "oic-main-content flex-1 lg:mr-5 mb-5 bg-white px-5
                        {content_cls}",
                    &cx.props.children
                }
                div {
                    class: "oic-page-side lg:w-96 relative",
                    BloodRecommend {}
                    RandomRecommends {}
                    HotTags {}
                    SiteInfo {}
                }
            }
        }
    ))
}