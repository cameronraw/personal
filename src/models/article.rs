use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub document_id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub content: String,
    pub image: Option<Image>,
    pub tags: Option<String>,
    pub published_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Article {
    pub fn get_tags(&self) -> Option<Vec<String>> {
        self.tags
            .as_ref()
            .map(|tags| tags.split(',').map(|tag| tag.to_string()).collect())
    }

    pub fn get_published_date(&self) -> Option<String> {
        if let Some(published_at) = &self.published_at {
            chrono::NaiveDateTime::parse_from_str(published_at, "%Y-%m-%dT%H:%M:%S%.fZ")
                .ok()
                .map(|datetime| datetime.format("%d/%m/%Y").to_string())
        } else {
            None
        }
    }

    pub fn get_updated_date(&self) -> Option<String> {
        if let Some(updated_at) = &self.updated_at {
            chrono::NaiveDateTime::parse_from_str(updated_at, "%Y-%m-%dT%H:%M:%S%.fZ")
                .ok()
                .map(|datetime| datetime.format("%d/%m/%Y").to_string())
        } else {
            None
        }
    }
}
#[derive(Deserialize)]
pub struct ArticleResponse {
    pub data: Article,
}
#[derive(Deserialize)]
pub struct ArticlesResponse {
    pub data: Vec<Article>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub formats: HashMap<String, ImageFormat>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageFormat {
    pub name: String,
    pub mime: String,
    pub width: u32,
    pub height: u32,
    pub url: String,
}
