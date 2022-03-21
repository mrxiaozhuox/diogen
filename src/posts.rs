use std::collections::HashMap;

use markdown_meta_parser::Value;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

pub async fn get_post_index() -> Vec<String> {
    if let Ok(resp) = Request::get("/posts/index.json").send().await {
        let r = resp.json::<Vec<String>>().await;
        if r.is_err() {
            return vec![];
        } else {
            return r.unwrap();
        }
    }
    vec![]
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

pub async fn get_post(name: &str, raw_path: &str) -> Option<ArticleInfo> {
    let resp = Request::get(&format!("{raw_path}posts/{name}"))
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

    let mut content = res.1.replace("\n", "");

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
        content: res.1
    })
}
