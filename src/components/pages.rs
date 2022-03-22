use dioxus::prelude::*;
use dioxus_heroicons::{solid::Shape, Icon};

use crate::{
    components::link::Link,
    config::DiogenConfig,
    posts::{get_post, get_post_index},
};

pub fn HomePage(cx: Scope) -> Element {
    let config = use_context::<DiogenConfig>(&cx).unwrap();
    let config = config.read();

    let repo = config.repository.clone().unwrap();
    let config = config.clone();
    let v = use_future(&cx, (), |_| async move {
        let list = get_post_index().await;
        let mut result = vec![];
        for ar in list {
            // raw_path 也要考虑子目录的问题
            let raw_path = config.root.clone();
            let meta = get_post(&ar, &raw_path).await;

            if let Some(meta) = meta {
                result.push(meta);
            } else if repo.substitute {
                // 如果直接获取 POST Meta 失败，尝试使用 Repo
                let meta = get_post(&ar, &repo.get_raw_path().unwrap()).await;
                if let Some(meta) = meta {
                    result.push(meta);
                }
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
                        div {
                            class: "card-content",
                            div {
                                class: "media",
                                div {
                                    class: "media-content",
                                    p {
                                        class: "title is-4",
                                        Link {
                                            to: "/p/{meta.sign_name}",
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
    let repo = config.repository.clone().unwrap();
    let config = config.clone();
    let r = use_future(&cx, (), |_| {
        async move {
            let info = if articles.contains_key(&sign_name) {
                Some(articles.get(&sign_name).unwrap().clone())
            } else {
                let file_name = base64::decode(sign_name.as_str()).unwrap_or_default();
                let file_name = String::from_utf8(file_name).unwrap_or_default();

                let raw_path = config.root.clone();
                let meta = get_post(&file_name, &raw_path).await;

                if meta.is_none() {
                    // if can't load meta from current path, try to load from repo.
                    let meta = get_post(&file_name, &repo.get_raw_path().unwrap()).await;
                    meta
                } else {
                    meta
                }
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
        storage_info.write().cache_article(&info.sign_name, info.clone());
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
