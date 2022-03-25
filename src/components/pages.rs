use dioxus::prelude::*;
use dioxus_heroicons::{solid::Shape, Icon};

use crate::{
    components::link::Link, config::DiogenConfig, posts::PostGetter, storage::StorageInfo,
};

pub fn HomePage(cx: Scope) -> Element {
    let config = use_context::<DiogenConfig>(&cx).unwrap();
    let config = config.read();

    // let repo = config.repository.clone().unwrap();
    let config = config.clone();
    let v = use_future(&cx, (), |_| async move {
        let post_getter = PostGetter {
            config: config.clone(),
        };
        let list = post_getter.get_post_index().await;
        let mut result = vec![];
        for ar in list {
            let meta = post_getter.get_post(&ar).await;
            if let Some(meta) = meta {
                result.push(meta);
            }
        }
        result
    });

    let storage_info = use_context::<crate::storage::StorageInfo>(&cx).unwrap();

    let article_list = match v.value() {
        Some(res) => {
            let ls = res.iter().rev().map(|meta| {
                let meta = meta.clone();

                // 这里需要统计 tags 和 category 用于渲染 tags list 和 category list 页面
                let tags = meta.tags.join("  ");

                storage_info
                    .write()
                    .cache_article(&meta.sign_name, meta.clone());

                storage_info
                    .write()
                    .cache_tags(&meta.sign_name, meta.tags.clone());
                storage_info
                    .write()
                    .cache_category(&meta.sign_name, meta.category.clone());

                storage_info.read().storage_all();

                rsx! {
                    div {
                        class: "card",
                        "key": "123",
                        div {
                            class: "card-content",
                            div {
                                class: "media",
                                div {
                                    class: "media-content",
                                    p {
                                        class: "title is-4",
                                        Link {
                                            to: "/articles/{meta.sign_name}",
                                            "{meta.title}"
                                        }
                                    }
                                    p {
                                        class: "subtitle is-6",
                                        "# {meta.date}"
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
                                        "{meta.category}"
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
                nav {
                    class: "pagination is-centered is-rounded",
                    role: "navigation",
                    a {
                        class: "pagination-previous",
                        "Previous"
                    }
                    a {
                        class: "pagination-next",
                        "Next page"
                    }
                }
                br {}
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

#[inline_props]
pub fn ArticleDisplay(cx: Scope, sign_name: String) -> Element {
    let config = use_context::<DiogenConfig>(&cx).unwrap();
    let config = config.read();

    let storage_info = use_context::<crate::storage::StorageInfo>(&cx).unwrap();
    let info = storage_info.read();
    let articles = info.article_content.clone();
    drop(info);

    let sign_name = sign_name.clone();
    let config = config.clone();
    let r = use_future(&cx, (), |_| {
        let post_getter = PostGetter {
            config: config.clone(),
        };
        async move {
            let file_name = base64::decode(sign_name.as_str()).unwrap_or_default();
            let file_name = String::from_utf8(file_name).unwrap_or_default();
            let info = if articles.contains_key(&sign_name) {
                let temp = articles.get(&sign_name).unwrap().clone();
                let temp = if temp.content.is_empty() {
                    post_getter.get_post(&file_name).await
                } else {
                    Some(temp)
                };
                temp
            } else {
                post_getter.get_post(&file_name).await
            };
            info
        }
    });

    if let Some(info) = r.value() {
        // if article is none, render 404 page.
        if info.is_none() {
            return cx.render(rsx! {
                crate::pages::_404 {}
            });
        }

        let info = info.as_ref().unwrap();
        storage_info
            .write()
            .cache_article(&info.sign_name, info.clone());
        return cx.render(rsx! {
            div {
                class: "container",
                section {
                    class: "hero is-info",
                    div {
                        class: "hero-body",
                        p {
                            class: "title",
                            "{info.title}"
                        }
                        p {
                            class: "subtitle",
                            "{info.date}"
                        }
                    }
                }
                br {}
                div {
                    class: "card",
                    div {
                        class: "card-content",
                        div {
                            class: "content",
                            id: "content",
                        }
                        div {
                            id: "meta-content",
                            style: "display: none;",
                            "{info.content}"
                        }
                    }
                }
                script {
                    src: "./assets/article.js"
                }
            }
        });
    } else {
        None
    }
}

pub fn _404(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            style: "text-align: center;",
            h3 {
                class: "title is-3",
                "404 Not Found"
            }
            p {
                h5 {
                    class: "subtitle is-5",
                    Link {
                        to: "/",
                        "To Home Page"
                    }
                }
            }
        }
    })
}

pub fn Tags(cx: Scope) -> Element {
    let storage_info = use_context::<StorageInfo>(&cx).unwrap();
    let storage_info = storage_info.read();

    let tags = &storage_info.tags;

    if tags.is_empty() {
        return cx.render(rsx! {
            div {
                class: "container",
                div {
                    class: "card",
                    div {
                        class: "card-content",
                        style: "text-align: center;",
                        strong { "No tags archive" }
                    }
                }
                br {}
            }
        });
    }

    let mut tag_vec = tags
        .iter()
        .map(|(k, v)| (k.clone(), v.len()))
        .collect::<Vec<_>>();
    tag_vec.sort_by(|a, b| b.1.cmp(&a.1));

    let tags_list = tag_vec.iter().map(|tag| {
        let name = tag.0.to_string();
        let num = tag.1;

        let class = "tag is-link";

        rsx! {
            li {
                a {
                    strong {
                        "{name}",
                    }
                    span {
                        style: "float: right;",
                        class: "{class}",
                        "{num}"
                    }
                }
            }
        }
    });

    cx.render(rsx! {
        div {
            class: "container",
            div {
                class: "card",
                div {
                    class: "card-content",
                    aside {
                        "class": "menu",
                        p {
                            class: "menu-label",
                            "Article Tags Archive"
                        }
                        ul {
                            class: "menu-list",
                            tags_list
                        }
                    }

                }
            }
            br {}
        }
    })
}

pub fn Category(cx: Scope) -> Element {
    let storage_info = use_context::<StorageInfo>(&cx).unwrap();
    let storage_info = storage_info.read();

    let category_list = use_state(&cx, || {
        let mut category_vec = storage_info
            .category
            .iter()
            .map(|(k, v)| (k.to_string(), v.len()))
            .collect::<Vec<_>>();
        category_vec.sort_by(|a, b| {
            if a.0 == "Default" {
                return a.1.cmp(&b.1);
            }
            b.1.cmp(&a.1)
        });
        category_vec
    });
    let current_category = use_state(&cx, || "Default".to_string());

    let display_list = if category_list.get().len() > 10 {
        category_list.get()[0..10].to_vec()
    } else {
        category_list.get().clone()
    };

    let display_list = display_list.iter().map(|(name, _)| {
        let n = name.clone();
        let class = if &n == current_category.get() {
            "is-active"
        } else {
            ""
        };
        rsx! {
            a {
                class: "{class}",
                onclick: move |_| {
                    current_category.set(n.clone());
                },
                "{name}"
            }
        }
    });

    let current_articles = storage_info
        .category
        .get(current_category.get())
        .cloned()
        .unwrap_or_default();
    let articles_list = current_articles.iter().map(|name| {
        let article = storage_info.article_content.get(name).unwrap();
        rsx! {
            Link {
                class: "panel-block",
                to: "/articles/{article.sign_name}",
                "@{article.title} - [{article.date}]"
            }
        }
    });

    cx.render(rsx! {
        div {
            class: "container",
            article {
                class: "panel is-info",
                p {
                    class: "panel-heading",
                    "Category Filter"
                }
                div {
                    class: "panel-block",
                    p {
                        class: "control has-icons-left",
                        input {
                            class: "input is-info",
                            r#type: "text",
                            placeholder: "Category Name",
                        }
                        span {
                            class: "icon is-small is-left",
                            Icon {
                                icon: Shape::Search,
                            }
                        }
                    }
                }
                p {
                    class: "panel-tabs",
                    display_list
                }
                articles_list
            }
        }
    })
}
