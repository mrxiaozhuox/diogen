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
        let mut link = (String::new(), v.link.clone());
        if &link.1 == "/" {
            link = (String::from("/#/"), "/".into());
        } else if link.1.starts_with('/') {
            let route_link = link.1;
            link = (format!("/#{route_link}"), v.link.clone());
        } else {
            link = (v.link.clone(), String::from("/@skip"));
        }

        rsx! {
            a {
                class: "navbar-item",
                href: "{link.0}",
                onclick: move |_| {
                    use_set(&cx, super::ROUTER)(link.1.clone());
                },
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
