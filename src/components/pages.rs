use dioxus::prelude::*;
use dioxus_heroicons::{Icon, solid::Shape};

use crate::{
    components::link::Link,
    config::DiogenConfig,
    posts::{get_post_index, get_post_meta},
};

pub fn HomePage(cx: Scope) -> Element {
    let config = use_context::<DiogenConfig>(&cx).unwrap();
    let config = config.read();

    let repo = config.repository.clone().unwrap();
    return cx.render(rsx! {
        [ repo.get_raw_path().unwrap() ]
    });

    let v = use_future(&cx, (), |_| async {
        let list = get_post_index().await;
        let mut result = vec![];
        for ar in list {
            let meta = get_post_meta(&ar).await;
            if let Some(meta) = meta {
                result.push(meta);
            }
        }
        result
    });

    let article_list = match v.value() {
        Some(res) => {
            let ls = res
                .iter()
                .map(|meta| {
                    let meta = meta.clone();
                    let tags = meta.tags.join(" , ");
                    let categories = meta.categories.join(" | ");
                    rsx! {
                        div {
                            class: "card",
                            div {
                                class: "card-content",
                                div {
                                    class: "media",
                                    div {
                                        class: "media-content",
                                        p {
                                            class: "title is-4",
                                            Link {
                                                to: "/post/1",
                                                "{meta.title}"
                                            }
                                        }
                                        p {
                                            class: "subtitle is-6",
                                            "Date - {meta.date}"
                                        }
                                    }
                                }
                                div {
                                    class: "content",
                                    "{meta.description}"
                                }
                            }
                            div {
                                class: "card-footer",
                                p {
                                    class: "card-footer-item",
                                    span {
                                        class: "icon-text",
                                        span {
                                            class: "icon",
                                            Icon {
                                                icon: Shape::Tag,
                                                size: 17,
                                            }
                                        }
                                        span {
                                            "{tags}"
                                        }
                                    }
                                }
                                p {
                                    class: "card-footer-item",
                                    span {
                                        class: "icon-text",
                                        span {
                                            class: "icon",
                                            Icon {
                                                icon: Shape::Archive,
                                                size: 17,
                                            }
                                        }
                                        span {
                                            "{categories}"
                                        }
                                    }
                                }
                            }
                        }
                        br {}
                    }
                });
            cx.render(rsx! {
                ls
            })
        }
        None => cx.render(rsx! {
            div {
                style: "text-align: center;",
                strong { "Loading..." }
            }
        }),
    };

    cx.render(rsx! {
        div {
            class: "container",
            article_list
        }
    })
}
