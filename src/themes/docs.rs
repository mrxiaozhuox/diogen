use dioxus::prelude::*;

pub fn Homepage(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "docs" }
    })
}
