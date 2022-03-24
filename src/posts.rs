use std::collections::HashMap;

use markdown_meta_parser::Value;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

use crate::config::DiogenConfig;

pub struct PostGetter {
    pub config: DiogenConfig,
}

impl PostGetter {
    #[allow(clippy::let_and_return)]
    fn get_root_path(&self) -> String {

        let deploy_scheme = self.config.deploy.current_scheme();
        let deploy_scheme = match deploy_scheme {
            Some(v) => v,
            None => return String::from("/"),
        };
        let request = deploy_scheme.request;

        let repo_info = &self.config.repository;
        let mut repo_raw_path = match repo_info {
            Some(v) => v.get_raw_path().unwrap(),
            None => String::new(),
        };
        if repo_raw_path.ends_with('/') {
            repo_raw_path = repo_raw_path[..repo_raw_path.len() - 1].to_string();
        }

        let result = request;
        let result = result.replace("#{repo:raw}", &repo_raw_path);

        let result = if result.ends_with('/') {
            result[..result.len() - 1].to_string()
        } else {
            result
        };

        result
    }

    pub async fn get_post_index(&self) -> Vec<String> {
        let url = format!("{}/posts/index.json", self.get_root_path());
        if let Ok(resp) = Request::get(&url).send().await {
            let r = resp.json::<Vec<String>>().await;
            if r.is_err() {
                return vec![];
            } else {
                return r.unwrap();
            }
        }
        vec![]
    }

    pub async fn get_post(&self, name: &str) -> Option<ArticleInfo> {
        let raw_path = self.get_root_path();
        let resp = Request::get(&format!("{raw_path}/posts/{name}"))
            .send()
            .await;
        if resp.is_err() {
            return None;
        }
        let resp = resp.unwrap();
        if resp.status() != 200 {
            return None;
        }
    
        let text = resp.text().await.unwrap();
    
        let mut type_mark = HashMap::new();
        type_mark.insert("tags".into(), "Array");
        let meta = markdown_meta_parser::MetaData {
            content: text,
            required: vec![],
            type_mark,
        };
    
        let res = meta.parse();
        if res.is_err() {
            return None;
        }
        let res = res.unwrap();
    
        let mut content = res.1.replace('\n', "");
    
        let temp = content
            .chars()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        if temp.len() > 350 {
            content = temp[0..350].concat();
        }
    
        let map = res.0;
    
        let title =
            if let Value::String(s) = map.get("title").unwrap_or(&Value::String(name.to_string())) {
                s.clone()
            } else {
                name.to_string()
            };
        let date = if let Value::String(s) = map
            .get("date")
            .unwrap_or(&Value::String("Unknown Date".to_string()))
        {
            s.clone()
        } else {
            "Unknown Date".to_string()
        };
        let tags = if let Value::Array(arr) = map.get("tags").unwrap_or(&Value::Array(vec![])) {
            arr.clone()
        } else {
            vec![]
        };
        let description = if let Value::String(s) = map
            .get("description")
            .unwrap_or(&Value::String(content.to_string()))
        {
            s.clone()
        } else {
            content
        };
        let category = if let Value::String(s) = map
            .get("category")
            .unwrap_or(&Value::String("Default".to_string()))
        {
            s.clone()
        } else {
            String::from("Default")
        };
    
        let sign_name = base64::encode(name.as_bytes());
    
        Some(ArticleInfo {
            sign_name,
            title,
            date,
            tags,
            description,
            category,
            content: res.1,
        })
    }
    

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleInfo {
    pub sign_name: String,
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub description: String,
    pub category: String,
    pub content: String,
}