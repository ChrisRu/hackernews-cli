use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct User {
    pub about: Option<String>,
    pub created_time: i32,
    pub created: String,
    pub id: String,
    pub karma: i32,
}

#[derive(Clone, Deserialize)]
pub struct Story {
    pub id: i32,
    pub title: String,
    pub points: Option<i32>,
    pub user: Option<String>,
    pub time: i64,
    pub time_ago: String,
    pub comments_count: i32,
    pub r#type: Option<String>,
    pub url: Option<String>,
    pub domain: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct StoryDetails {
    pub id: i32,
    pub title: Option<String>,
    pub points: Option<i32>,
    pub user: Option<String>,
    pub time: i32,
    pub time_ago: String,
    pub content: String,
    pub deleted: Option<bool>,
    pub dead: Option<bool>,
    pub r#type: String,
    pub url: Option<String>,
    pub domain: Option<String>,
    pub comments: Vec<StoryDetails>,
    pub level: Option<i32>,
    pub comments_count: i32,
}
