#![allow(non_snake_case)]

mod config;
mod router;
mod themes;
mod component;

use dioxus::prelude::*;

use crate::{config::DiogenConfig, themes::TopBar};

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
    // let window = web_sys::window().unwrap();
    // let web_document = window.document().unwrap();

    // 全局 Router 数据，使用 Fermi 状态管理工具
    let router = use_read(&cx, ROUTER);

    // 当 APP 组件你第一次被运行时，会加载配置文件并更新信息
    let config = use_state(&cx, || {
        js_sys::eval("window.diogen")
            .unwrap()
            .into_serde::<DiogenConfig>()
            .unwrap_or_else(|e| {
                js_sys::eval("alert('Diogen config load failed.')").unwrap();
                panic!("{}", e);
            })
    })
    .get()
    .clone();

    // 因为我们会常常使用到 config 信息，所以说我们通过 context 将它传递
    // 这种方案要比 props 传递更加方便
    use_context_provider(&cx, || config.clone());

    let Homepage = if &config.theme == "blog" {
        themes::blog::Homepage
    } else {
        themes::docs::Homepage
    };

    cx.render(rsx! {
        style { "html::-webkit-scrollbar {{display: none;}}" }

        TopBar {}

        // 自制路由系统，因为 Dioxus 原生的路由无法满足我的需求
        // 我需要的路径控制主要在 /#/ 之后
        match router.as_str() {
            "/" => {
                rsx! {
                    Homepage {}
                }
            }
            "/@skip" => {
                rsx! {
                    div { "S" }
                }
            }
            _v => {
                rsx! { "404 Not Found" }
            }
        }
    })
}
