use dioxus::prelude::*;

use crate::{config::DiogenConfig, component::Link};

pub fn Homepage(cx: Scope) -> Element {
    let config = use_context::<DiogenConfig>(&cx).unwrap();
    let config = config.read();

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
