use dioxus::prelude::*;
use super::{Banner, Article, Projects, Roster};
use crate::components::layouts::{
    Header, Footer, PageMain, MainSection,
};
use crate::types::MenuId;

pub fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "oic-homePage-container",
            Header {
                active_vid: MenuId::Home.get_vid(),
            }
            PageMain {
                class: "oic-home-main",
                Banner {}
                MainSection {
                    div {
                        class: "oic-article-list",
                        (1..10).map(|_| {
                            rsx! (
                                Article {}
                            )
                        })
                    }
                }
                Projects {}

                Roster {}
            }
            Footer {}
        }
    })
}