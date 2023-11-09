use dioxus::prelude::*;
use crate::types::HotTagItem;

pub fn HotTags(cx: Scope) -> Element {
    let tags: Vec<HotTagItem> = vec![
        HotTagItem { name: String::from("可穿戴"), num: 3 },
        HotTagItem { name: String::from("电源"), num: 2 },
        HotTagItem { name: String::from("移动电源"), num: 4 },
        HotTagItem { name: String::from("手环"), num: 2 },
        HotTagItem { name: String::from("摩托车"), num: 1 },
        HotTagItem { name: String::from("智能眼镜"), num: 1 },
        HotTagItem { name: String::from("隐形眼镜"), num: 1 },
        HotTagItem { name: String::from("运动"), num: 1 },
        HotTagItem { name: String::from("可穿戴"), num: 1 },
        HotTagItem { name: String::from("可穿戴"), num: 1 },
        HotTagItem { name: String::from("可穿戴"), num: 1 },
        HotTagItem { name: String::from("可穿戴"), num: 1 },
        HotTagItem { name: String::from("可穿戴"), num: 3 },
        HotTagItem { name: String::from("可穿戴"), num: 3 },
        HotTagItem { name: String::from("可穿戴"), num: 3 },
        HotTagItem { name: String::from("可穿戴"), num: 3 },
        HotTagItem { name: String::from("可穿戴"), num: 3 },
        HotTagItem { name: String::from("可穿戴"), num: 3 },
        HotTagItem { name: String::from("可穿戴"), num: 3 },
        HotTagItem { name: String::from("可穿戴"), num: 3 },
    ];

    cx.render(rsx!(
        div {
            class: "oic-widget oic-widget-hotTags",
            h3 {
                class: "oic-widget-title",
                "热门标签"
            }
            div {
                class: "oic-hot-tags",
                div {
                    class: "oic-inner mr-[-8px]",
                    tags.iter().map(|item| {
                        rsx!(
                            a {
                                class: "oic-tag-item opacity-80
                                    text-white inline-block px-2
                                    hover:opacity-100
                                    mr-2 mb-2 text-sm leading-7",
                                item.name.as_str()"({item.num})"
                            }
                        )
                    })
                }
            }
        }
    ))
}
