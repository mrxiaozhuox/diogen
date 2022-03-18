use dioxus::prelude::*;

use crate::config::DiogenConfig;

#[derive(Props)]
pub struct LinkProps<'a> {
    #[props(default = "#")]
    pub to: &'a str,

    #[props(default)]
    pub class: &'a str,

    #[props(default = "_self")]
    pub target: &'a str,

    pub children: Element<'a>,
}

pub fn Link<'a>(cx: Scope<'a, LinkProps<'a>>) -> Element {
    let config = use_context::<DiogenConfig>(&cx).unwrap();
    let config = config.read();

    let to = cx.props.to.to_string();

    let mut link = (String::new(), to.clone());
    if link.1.starts_with('/') {
        let route_link = link.1;
        link = (format!("/#{route_link}"), to);

        if &config.root != "/" && !config.root.is_empty() {
            if link.0.starts_with('/') {
                link.0 = link.0[1..].to_string();
            }
            link.0 = if config.root.ends_with('/') {
                format!("{}{}", config.root, link.0)
            } else {
                format!("{}/{}", config.root, link.0)
            }
        }
    } else {
        link = (to, String::from("/@skip"));
    }

    cx.render(rsx! {
        a {
            class: "{cx.props.class}",
            href: "{link.0}",
            target: "{cx.props.target}",
            onclick: move |_| {
                use_set(&cx, crate::ROUTER)(link.1.clone());
            },
            &cx.props.children
        }
    })
}
