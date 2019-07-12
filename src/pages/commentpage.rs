
use crate::fetch::get_story;
use crate::models::StoryDetails;
use crate::print::print_story;
use crate::pages::Page;

pub struct CommentPage {
    story_id: i32,
    story: Option<StoryDetails>,
}

impl CommentPage {
    pub fn new(story_id: i32) -> CommentPage {
        CommentPage {
            story_id,
            story: None,
        }
    }
}

impl Page for CommentPage {
    fn render(&self) {
        print_story(self.story.clone().unwrap());
    }

    fn fetch_data(&mut self) {
        self.story = Some(get_story(&self.story_id.to_string()));
    }

    fn input(&mut self, _key: i32) {}
}
