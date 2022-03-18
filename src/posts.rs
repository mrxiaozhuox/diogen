use std::collections::HashMap;

use markdown_meta_parser::Value;
use reqwasm::http::Request;

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

#[derive(Debug, Clone)]
pub struct ArticleMeta {
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub description: String,
    pub categories: Vec<String>,
}

pub async fn get_post_meta(name: &str) -> Option<ArticleMeta> {
    let resp = Request::get(&format!("/posts/{name}")).send().await;
    if resp.is_err() {
        return None;
    }
    let resp = resp.unwrap();

    let text = resp.text().await.unwrap();

    let mut type_mark = HashMap::new();
    type_mark.insert("tags".into(), "Array");
    type_mark.insert("categories".into(), "Array");
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

    if content.len() > 350 {
        let temp = content.chars().into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
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
    let categories =
        if let Value::Array(arr) = map.get("categories").unwrap_or(&Value::Array(vec![])) {
            arr.clone()
        } else {
            vec![]
        };

    Some(ArticleMeta {
        title,
        date,
        tags,
        description,
        categories,
    })
}
