#![allow(non_snake_case)]

mod components;
mod config;
mod posts;
mod repository;
mod router;
mod storage;
mod theme;

use crate::components::{nav::TopBar, pages};
use dioxus::prelude::*;
use reqwasm::http::Request;
use storage::StorageInfo;

use crate::config::DiogenConfig;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch(app);
}

static ROUTER: Atom<String> = |_| {
    let window = web_sys::window().unwrap();
    let web_document = window.document().unwrap();
    router::url_to_route(&web_document.url().unwrap())
};

fn app(cx: Scope) -> Element {
    // 全局 Router 数据，使用 Fermi 状态管理工具
    let router = use_read(&cx, ROUTER);

    // 当 APP 组件第一次被运行时，会加载配置文件并更新信息
    let config = use_future(&cx, (), |()| async move {
        if let Ok(resp) = Request::get("/diogen.json").send().await {
            let res = resp.json::<DiogenConfig>().await;
            if res.is_err() {
                js_sys::eval("alert('load config failed.')").unwrap();
                let error = res.err().unwrap();
                panic!("load config failed: {error}");
            }
            res.unwrap()
        } else {
            js_sys::eval("alert('load config failed.')").unwrap();
            panic!("load config failed");
        }
    });

    config.value()?;

    // 因为会常常使用到 config 信息，所以说通过 context 将它传递
    // 这种方案要比 props 传递更加方便
    let config = config.value().unwrap();
    use_context_provider(&cx, || config.clone());
    use_context_provider(&cx, StorageInfo::default);

    cx.render(rsx! {
        TopBar {}

        // 自制路由系统，因为 Dioxus 原生的路由无法满足我的需求
        // 我需要的路径控制主要在 /#/ 之后
        match router.as_str() {
            "/" => {
                cx.render(
                    rsx! {
                        pages::HomePage {}
                    }
                )
            }
            "/@skip" => {
                cx.render(
                    rsx! {
                        div {
                            style: "text-align: center;",
                            "Waiting for the jump!"
                        }
                    }
                )
            }
            path => {
                cx.render(
                    rsx! {
                        router::DynPath {
                            path: path
                        }
                    }
                )
            }
        }

        footer {
            class: "footer",
            div {
                class: "content has-text-centered",
                p {
                    "Powered by "
                    strong {
                        a {
                            href: "https://diogen.mrxzx.info/",
                            "Diogen"
                        }
                    }
                }
            }
        }

    })
}
