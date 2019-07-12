
use crate::models::Story;
use crate::models::StoryDetails;

const STORIES_URL: &str = "https://api.hnpwa.com/v0/news/";
const STORY_URL: &str = "https://api.hnpwa.com/v0/item/";
const URL_APPEND: &str = ".json";

pub fn get_stories(path: &str) -> Vec<Story> {
    reqwest::get(&[STORIES_URL, path, URL_APPEND].join(""))
        .unwrap()
        .json()
        .unwrap()
}

pub fn get_story(path: &str) -> StoryDetails {
    reqwest::get(&[STORY_URL, path, URL_APPEND].join(""))
        .unwrap()
        .json()
        .unwrap()
}
