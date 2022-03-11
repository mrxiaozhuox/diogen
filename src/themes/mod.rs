use dioxus::prelude::*;

use crate::config::DiogenConfig;

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
            a {
                class: "navbar-item",
                href: "/#{v.link}",
                onclick: |_| { js_sys::eval("location.reload()").unwrap(); },
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
