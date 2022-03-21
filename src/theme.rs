use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ThemeInfo {
    pub about: ThemeAbout,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ThemeAbout {
    #[serde(default = "theme_about_type_default")]
    pub r#type: String,

    #[serde(default)]
    pub title: String,

    #[serde(default)]
    pub subtitle: String,

    #[serde(default)]
    pub url: String,
}

fn theme_about_type_default() -> String {
    String::from("text")
}
