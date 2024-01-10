use dioxus::prelude::*;

pub fn Roster(cx: Scope) -> Element {
    cx.render(rsx! (
        section {
            class: "oic-home-roster bg-purple-800",
            div {
                class: "oic-inner py-20 mx-6 md:mx-auto max-w-7xl flex
                    flex-col items-center justify-center",
                h2 {
                    class: "text-white font-bold text-2xl md:text-4xl
                        text-center",
                    "不求大师的水平，只需分享、参与的热情！"
                }
                div {
                    class: "text-gray-400 mt-6 mx-auto md:w-2/3 text-center
                        text-lg",
                    "几乎能为任何应用程序或需求自动地作出优化和定制； 对极限的配置、性能的追求，顶尖的用户和开发者；一起交流，解决问题，交流技术， 提高普及率。"
                }
                div {
                    class: "flex flex-row justify-center mt-6",
                    a {
                        class: "oic-btn-default md:min-w-[15rem]",
                        "参与建设"
                    }
                    a {
                        class: "oic-btn-simple ml-5 md:min-w-[15rem]",
                        "我能做些什么？"
                    }
                }
            }
        }
    ))
}