use serde::{Deserialize, Serialize};

use crate::deploy::DeployInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiogenConfig {
    pub title: String,
    pub author: String,

    #[serde(default = "default_root")]
    pub root: String,

    #[serde(default)]
    pub debug: bool,

    #[serde(default = "deploy_default")]
    pub deploy: DeployInfo,

    #[serde(default)]
    pub theme: crate::theme::ThemeInfo,

    #[serde(default)]
    pub repository: Option<crate::repository::RepositoryInfo>,

    #[serde(default)]
    pub nav: Vec<NavInfo>,
}

fn deploy_default() -> DeployInfo {
    DeployInfo { scheme: Default::default() }
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
