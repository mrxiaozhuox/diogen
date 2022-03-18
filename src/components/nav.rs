use dioxus::prelude::*;
use dioxus_heroicons::Icon;
use crate::components::link::Link;
use crate::config::DiogenConfig;

use super::icons;

#[inline_props]
pub fn TopBar(cx: Scope) -> Element {
    let config = use_context::<DiogenConfig>(&cx).unwrap();
    let config = config.read();

    // log::info!("{config:?}");

    let curr_route = use_read(&cx, crate::ROUTER);

    let nav_list = config.nav.clone();
    let nav_list = nav_list.iter().map(|v| {
        let class = if curr_route == &v.link {
            "navbar-item is-active"
        } else {
            "navbar-item"
        };

        
        let shape = icons::get_solid_icon(&v.icon);
        if shape.is_none() {
            rsx! {
                Link {
                    class: "{class}",
                    to: "{v.link}",
                    "{v.text}"
                }
            }
        } else {
            rsx! {
                Link {
                    class: "{class}",
                    to: "{v.link}",
                    Icon {
                        icon: shape.unwrap(),
                        size: 16,
                    }
                    "{v.text}"
                }
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
                        href: "javascript:;",
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
        br {}
    ))
}
