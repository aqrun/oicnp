use dioxus::prelude::*;

pub fn ItemFooter(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "oic-article-footer hidden flex-wrap
                lg:flex flex-col lg:flex-row justify-between
                lg:items-center text-sm",
            div {
                class: "oic-col-left flex flex-col lg:flex-row lg:items-center
                    flex-wrap",
                // 作者和发布时间
                AuthorAndDate {}
                // 额外信息：分类 阅读量
                ExtraData {}
            }
            div {
                class: "oic-col-right mt-2 lg:mt-0",
                // 标签
                TagList {}
            }
        }
    ))
}

/// 作者和发布时间
pub fn AuthorAndDate(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            class: "flex flex-row items-center",
            div {
                class: "oic-author-info flex items-center",
                a {
                    class: "block w-8 h-8 bg-slate-100 rounded-[50%]
                        relative overflow-hidden",
                    // 作者图片
                    img {

                    }
                }
                // 作者名称
                a {
                    class: "block ml-2 text-black",
                    "Alex"
                }
            }
            // 发布时间
            div {
                class: "oic-article-date ml-3 text-slate-600",
                "2023年02月07日 13:40"
            }
        }
    ))
}

/// 额外信息
/// 分类 阅读量 评论数 点赞数
pub fn ExtraData(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            class: "flex flex-row items-center mt-2 lg:mt-0",
            // 分类
            div {
                class: "oic-category lg:ml-6 flex items-center",
                i {
                    class: "icon iconfont icon-benshubook122 text-black",
                }
                span {
                    class: "ml-1 text-black-300 font-normal",
                    "JAVA"
                }
            }
            // 阅读
            div {
                class: "oic-viewed ml-6 flex items-center",
                i {
                    class: "icon iconfont icon-view2 text-black",
                }
                span {
                    class: "ml-1 text-black-300 font-light",
                    "104"
                }
            }
            // 评论
            div {
                class: "oic-commented ml-6 flex items-center",
                i {
                    class: "icon iconfont icon-comments text-black",
                }
                span {
                    class: "ml-1 text-black-300 font-light",
                    "14"
                }
            }
            // 点赞
            div {
                class: "oic-voted ml-6 flex items-center",
                i {
                    class: "icon iconfont icon-thumbs-o-up text-black",
                }
                span {
                    class: "ml-1 text-black-300 font-light",
                    "3"
                }
            }
        }
    ))
}

/// 标签列表
pub fn TagList(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            class: "oic-tags flex items-center text-black-300",
            i {
                class: "icon iconfont icon-tag",
            }
            div {
                class: "oic-tag-items flex items-center",
                a {
                    class: "ml-6 first:ml-2 hover:text-purple",
                    "Node.js"
                }
                a {
                    class: "ml-6 hover:text-purple",
                    "前端"
                }
                a {
                    class: "ml-6 hover:text-purple",
                    "JavaScript"
                }
            }
        }
    ))
}