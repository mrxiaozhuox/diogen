use dioxus::prelude::*;

use crate::config::DiogenConfig;

#[derive(Props)]
pub struct LinkProps<'a>{
    
    #[props(default = "#")]
    pub to: &'a str,

    #[props(default)]
    pub class: &'a str,

    #[props(default = "_blank")]
    pub target: &'a str,

    pub children: Element<'a>,
}

pub fn Link<'a>(cx: Scope<'a, LinkProps<'a>>) -> Element {
    
    let config = use_context::<DiogenConfig>(&cx).unwrap();
    let config = config.read();

    let mut to = cx.props.to.to_string();
    if &config.root != "/" && !config.root.is_empty() {
        to = if config.root.ends_with('/') {
            format!("{}{}", config.root, to)
        } else {
            format!("{}/{}", config.root, to)
        }
    }

    cx.render(rsx! {
        a {
            class: "{cx.props.class}",
            href: "{to}",
            target: "{cx.props.target}",
            &cx.props.children
        }
    })
}