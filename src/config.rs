use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiogenConfig {
    pub title: String,

    #[serde(default = "default_theme")]
    pub theme: String,

    #[serde(default)]
    pub nav: Vec<NavInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NavInfo {
    pub text: String,
    pub link: String,

    #[serde(default = "nav_default_target")]
    pub target: String,
}

fn default_theme() -> String {
    String::from("blog")
}

fn nav_default_target() -> String {
    String::from("_blank")
}
