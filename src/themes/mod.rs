use dioxus::prelude::*;

use crate::{config::DiogenConfig, component::Link};

pub mod blog;
pub mod docs;

#[inline_props]
pub fn TopBar(cx: Scope) -> Element {
    let config = use_context::<DiogenConfig>(&cx).unwrap();
    let config = config.read();

    // log::info!("{config:?}");

    let nav_list = config.nav.clone();
    let nav_list = nav_list.iter().map(|v| {
        rsx! {
            Link {
                class: "navbar-item",
                to: "{v.link}",
                "{v.text}"
            }
        }
    });

    cx.render(rsx!(
        nav {

            class: "navbar is-link is-fixed-top",
            role: "navigation",

            div {
                class: "container",
                div {
                    class: "navbar-brand",
                    a {
                        class: "navbar-item",
                        style: "font-size: 21px;",
                        href: "#",
                        strong { "{config.title}" }
                    }
                    a {
                        class: "navbar-burger",
                        role: "button",
                        "data-target": "navbarMenus",
                        span {}
                        span {}
                        span {}
                    }
                }

                div {
                    class: "navbar-menu",
                    id: "navbarMenus",
                    div {
                        class: "navbar-start",
                    }
                    div {
                        class: "navbar-end",
                        nav_list
                    }
                }
            }

        }
        br {}
        br {}
    ))
}
