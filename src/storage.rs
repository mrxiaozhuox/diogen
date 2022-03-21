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
                tags.push(article.to_string());
            } else {
                self.tags.insert(tag, vec![article.to_string()]);
            }
        }
    }

    pub fn cache_category(&mut self, article: &str, category: String) {
        if self.category.contains_key(&category) {
            let category = self.category.get_mut(&category).unwrap();
            category.push(article.to_string());
        } else {
            self.category.insert(category, vec![article.to_string()]);
        }
    }

    pub fn save_article(&mut self, sign_name: &str, article: ArticleInfo) {
        self.article_content.insert(sign_name.to_string(), article);
    }
}
