use serde::{Serialize, Deserialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepositoryInfo {
    #[serde(default = "repo_default_type")]
    pub r#type: String,

    pub url: String,

    #[serde(default = "repo_default_branch")]
    pub branch: String,
}

fn repo_default_type() -> String {
    String::from("github")
}
fn repo_default_branch() -> String {
    String::from("main")
}

impl RepositoryInfo {

    pub fn get_raw_path(&self) -> anyhow::Result<String> {
        
        let url = Url::parse(&self.url)?;

        Ok(
            match self.r#type.to_lowercase().as_str() {
                "github" => {
                    let url_path = url.path();
                    let mut new_url = Url::parse("https://raw.githubusercontent.com/")?;
                    new_url = new_url.join(url_path)?;
                    new_url = new_url.join(&format!("{}/", &self.branch))?;
                    new_url.to_string()
                },
                _ => url.to_string(),
            }
        )
    }
}