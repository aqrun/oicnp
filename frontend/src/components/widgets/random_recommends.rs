use dioxus::prelude::*;

pub fn RandomRecommends(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            class: "oic-widget oic-widget-randomRecommend",
            h3 {
                class: "oic-widget-title",
                "随机推荐"
            }
            ul {
                (0..5).map(|_| {
                    rsx! (
                        RandomItem {}
                    )
                })
            }
        }
    ))
}

fn RandomItem(cx: Scope) -> Element {
    cx.render(rsx!(
        li {
            class: "border-b border-solid border-slate-100 last:border-b-0",
            div {
                class: "flex items-center px-5 py-2 text-slate-800
                    text-sm hover:text-purple",
                div {
                    class: "oic-thumbnail rounded-md overflow-hidden
                        w-32 min-w-[8rem] h-20 bg-slate-100 mr-4",
                    img {}
                }
                div {
                    class: "oic-titleAndDate block",
                    a {
                        class: "text-justify block",
                        "显瘦牛仔裙半身裙短新款潮裙女夏韩版学生高腰a字裙"
                    }
                    span {
                        class: "block text-xs text-gray-400 mt-2",
                        "2018-10-25"
                    }
                }
            }
        }
    ))
}