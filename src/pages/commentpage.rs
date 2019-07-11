
use crate::fetch::get_story;
use crate::models::StoryDetails;
use crate::pages::Page;
use crate::print::print_story;
use crate::keys::is_exit;

pub struct CommentPage {
    story_id: i32,
    story: Option<StoryDetails>,
    on_close_comments: Box<Fn()>,
}

impl CommentPage {
    pub fn new(story_id: i32, on_close_comments: Box<Fn()>) -> CommentPage {
        CommentPage {
            story_id,
            story: None,
            on_close_comments: on_close_comments,
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

    fn input(&mut self, key: i32) {
        if is_exit(key) {
            (self.on_close_comments)();
        }
    }
}
