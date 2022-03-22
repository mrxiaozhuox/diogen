use dioxus::prelude::*;
use dioxus_heroicons::Icon;
use reqwasm::http::Request;
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

    let config = config.clone();
    let nav = cx.render(rsx!(
        nav {

            class: "navbar is-info is-fixed-top",
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
    ));


    if curr_route == "/" {
        let theme_info = &config.theme;
        let theme_about = theme_info.about.clone();

        let title = use_future(&cx, (), |_| {
            async move {
                if theme_about.r#type == "text" {
                    return (theme_about.title, theme_about.subtitle);
                } else if theme_about.r#type == "api:text" {
                    let api_url = theme_about.url.clone();
                    if let Ok(resp) = Request::get(&api_url).send().await {
                        if let Ok(str) = resp.text().await {
                            return (str, String::new());
                        }
                    }
                }
                (String::new(), String::new())
            }
        });

        let title = if let Some(v) = title.value() {
            v.clone()
        } else {
            (String::new(), String::new())
        };

        return cx.render(rsx! {
            nav
            section {
                class: "hero is-info is-fullheight is-fullheight-with-navbar",
                div {
                    class: "hero-body",
                    div {
                        class: "container has-text-centered",
                        p {
                            class: "title",
                            "{title.0}"
                        }
                        p {
                            class: "subtitle",
                            "{title.1}"
                        }
                    }
                }
                div {
                    class: "hero-foot",
                }
            }
            br {}
        });
    }

    cx.render(rsx! {
        nav   
        br {}
    })
}
