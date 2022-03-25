use dioxus::prelude::*;

pub fn url_to_route(url: &str) -> String {
    let url = url.to_string();

    let list = url.split("/#/").collect::<Vec<&str>>();

    if list.len() <= 1 {
        return String::from("/");
    }

    format!("/{}", list.get(1).unwrap())
}

#[inline_props]
pub fn DynPath<'a>(cx: Scope<'a>, path: &'a str) -> Element {

    if path.len() > 10 && &path[0..10] == "/articles/" {
        let sign_name = path[10..].to_string();
        return cx.render(rsx! {
            crate::components::pages::ArticleDisplay {
                sign_name: sign_name,
            }
        });
    }

    if *path == "/tags" {
        return cx.render(rsx! {
            crate::components::pages::Tags {}
        });
    } else if *path == "/category" {
        return cx.render(rsx! {
            crate::components::pages::Category {}
        });
    }

    cx.render(rsx! {
        crate::components::pages::_404 {}
    })
}