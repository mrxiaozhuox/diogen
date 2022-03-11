use dioxus::prelude::*;

use crate::props::HomePageProps;

pub fn Homepage(cx: Scope<HomePageProps>) -> Element {
    cx.render(rsx! {
        div { "docs" }
    })
}
