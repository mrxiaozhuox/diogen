use dioxus::prelude::*;

use crate::{components::link::Link, config::DiogenConfig};



pub fn HomePage(cx: Scope) -> Element {
    let config = use_context::<DiogenConfig>(&cx).unwrap();
    let _config = config.read();

    cx.render(rsx! {
        div {
            class: "container",
            style: "text-align:center;",
            div {
                class: "card",
                div {
                    class: "card-content",
                    Link {
                        to: "/post/test",
                        class: "title",
                        "Hello World"
                    }
                }
            }
        }
    })
}
