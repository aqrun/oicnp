use dioxus::prelude::*;
use super::ItemFooter;

pub fn Article(cx: Scope) -> Element {
    cx.render(rsx! (
        article {
            class: "oic-article-item mb-1 mx-[-20px] py-4 pl-5 pr-5 relative 
                border-b border-solid border-b-slate-200 last:border-b-0 
                hover:shadow-md",
            div {
                class: "oic-row flex items-start flex-nowrap justify-between",
                div {
                    class: "oic-article-content min-h-[6rem] flex 
                        flex-col justify-between
                        lg:min-h-0 lg:block",
                    h4 {
                        class: "oic-title",
                        a {
                            class: "break-words font-medium lg:font-bold
                                text-sm lg:text-lg lg:leading-6 mb-3 
                                overflow-ellipsis 
                                text-slate-800 hover:text-purple",
                            "看不懂源码？我总结了18条心法，助你修炼神功！"
                        }
                    }
                    div {
                        class: "oic-desc text-justify overflow-hidden
                            text-sm overflow-ellipsis h-11 break-words mt-4
                            hidden lg:block",
                        p {
                            "如何去阅读源码，18条心法祝你修炼神功！如何去阅读源码，18条心法祝你修炼神功！如何去阅读源码，18条心法祝你修炼神功！如何去阅读源码，18条心法祝你修炼神功 如何去阅读源码，18条心法祝你修炼神功！如何去阅读源码，18条心法祝你修炼神功！"
                        }
                    }
                }
                div {
                    class: "oic-article-img 
                    ml-5 w-44
                    min-h-[6rem] min-w-[8rem]
                    lg:min-h-[123px] lg:min-w-[180px]
                    relative bg-slate-100 rounded-md",
                }
            }
            ItemFooter {}
        }
    ))
}
