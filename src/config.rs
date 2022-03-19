use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiogenConfig {
    pub title: String,

    #[serde(default = "default_root")]
    pub root: String,

    #[serde(default)]
    pub repository: Option<crate::repository::RepositoryInfo>,

    #[serde(default)]
    pub nav: Vec<NavInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NavInfo {
    pub text: String,
    pub link: String,

    #[serde(default)]
    pub icon: String,

    #[serde(default = "nav_default_target")]
    pub target: String,
}

fn default_root() -> String {
    String::from("/")
}

fn nav_default_target() -> String {
    String::from("_blank")
}
