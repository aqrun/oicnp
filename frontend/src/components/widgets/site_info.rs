use dioxus::prelude::*;
use crate::types::SiteInfoItem;

pub fn SiteInfo(cx: Scope) -> Element {
    let infos: Vec<SiteInfoItem> = vec![
        SiteInfoItem { name: String::from("日志总数"), num: String::from("49") },
        SiteInfoItem { name: String::from("评论总数"), num: String::from("30") },
        SiteInfoItem { name: String::from("标签总数"), num: String::from("34") },
        SiteInfoItem { name: String::from("页面总数"), num: String::from("11") },
        SiteInfoItem { name: String::from("分类总数"), num: String::from("28") },
        SiteInfoItem { name: String::from("链接总数"), num: String::from("14") },
        SiteInfoItem { name: String::from("用户总数"), num: String::from("6128") },
        SiteInfoItem { name: String::from("最后更新"), num: String::from("2023-12-01") },
    ];

    cx.render(rsx! (
        div {
            class: "oic-widget oic-widget-siteInfo",
            h3 {
                class: "oic-widget-title",
                "网站统计"
            }
            ul {
                class: "py-3 px-5 leading-loose text-gray-500 flex flex-wrap",
                infos.iter().map(|item| {
                    rsx! (
                        li {
                            class: "w-1/2",
                            span {
                                class: "text-gray-800",
                                item.name.as_str()
                            }
                            span {
                                class: "inline-block mx-1",
                                ":"
                            }
                            span {
                                item.num.as_str()
                            }
                        }
                    )
                })
            }
        }
    ))
}