use dioxus::prelude::*;
use crate::get_url;

pub fn Banner(cx: Scope) -> Element {
    let banner_img = get_url("/assets/img/home-banner.avif");

    cx.render(rsx! {
        section {
            class: "oic-banner pb-20 pt-6 bg-white",
            div {
                class: "mx-auto lg:px-6 lg:w-11/12 max-w-7xl",
                div {
                    class: "flex flex-col-reverse lg:flex-row items-center gap-12",
                    BannerText {}
                    div {
                        class: "oic-header-img-w w-10/12 lg:w-1/2",
                        div {
                            class: "oic-image-w lg:max-w-[800px] relative",
                            img {
                                class: "max-w-full w-auto h-auto",
                                src: "{banner_img}",
                                alt: "banner",
                                width: "800px",
                                height: "456px",
                            }
                        }
                    }
                }
            }
        }
    })
}

pub fn BannerText(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "oic-header-text-w grid gap-6 justify-items-start w-10/12 lg:w-1/2 text-black",
            div {
                class: "grid gap-2 max-w-4xl",
                h1 {
                    class: "text-3xl lg:text-4xl font-bold text-black-500",
                    b {
                        class: "text-purple",
                        "爱而喜之"
                    }
                    br {}
                    "学而不思则惘，思而不学则殆"
                }
            }
            div {
                class: "lg:max-w-2xl whitespace-pre-wrap leading-normal text-xl",
                p {
                    "学而时习之，不亦说乎。知者乐水，仁者乐山。知者动，仁者静，知者乐，仁者寿。知之者不如好之者，好之者不如乐之者。吾日三省吾身：为人谋而不忠乎？与朋友交而下信乎？传不习乎？--孔子"
                }
            }
            div {
                class: "flex flex-row justify-center",
                a {
                    class: "oic-btn-default lg:min-w-[15rem]",
                    "前路漫漫"
                }
                a {
                    class: "oic-btn-simple ml-5 lg:min-w-[15rem] hover:underline
                        hover:underline-offset-4",
                    "充满希望"
                }
            }
        }
    })
}