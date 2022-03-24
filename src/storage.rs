use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::posts::ArticleInfo;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageInfo {
    pub tags: HashMap<String, Vec<String>>,
    pub category: HashMap<String, Vec<String>>,
    pub article_content: HashMap<String, ArticleInfo>,
}

#[allow(clippy::map_entry)]
impl StorageInfo {
    pub fn cache_tags(&mut self, article: &str, tags: Vec<String>) {
        for tag in tags {
            if self.tags.contains_key(&tag) {
                let tags = self.tags.get_mut(&tag).unwrap();
                if !tags.contains(&article.to_string()) {
                    tags.push(article.to_string());
                }
            } else {
                self.tags.insert(tag, vec![article.to_string()]);
            }
        }
    }

    pub fn cache_category(&mut self, article: &str, category: String) {
        if self.category.contains_key(&category) {
            let category = self.category.get_mut(&category).unwrap();
            if !category.contains(&article.to_string()) {
                category.push(article.to_string());
            }
        } else {
            self.category.insert(category, vec![article.to_string()]);
        }
    }

    pub fn cache_article(&mut self, sign_name: &str, article: ArticleInfo) {
        self.article_content.insert(sign_name.to_string(), article);
    }

    pub fn storage_all(&self) {
        let local_stroage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        let mut need_storage: StorageInfo = self.clone();
        // We don't need storage ArticleInfo into localstorage, because the data is very big!
        need_storage.article_content = HashMap::new();

        local_stroage
            .set_item(
                "diogen-archive",
                &serde_json::to_string(&need_storage).unwrap(),
            )
            .unwrap();
    }

    pub fn load_all() -> Option<Self> {
        let local_stroage = web_sys::window()?.local_storage().ok()??;
        let v = local_stroage.get_item("diogen-archive").ok()??;
        serde_json::from_str::<Self>(&v).ok()
    }
}
