
use crate::fetch::get_stories;
use crate::models::Story;
use crate::pages::Page;
use crate::print::print_stories;

use ncurses::*;

pub struct HomePage {
    stories: Vec<Story>,
    current_story: usize,
    on_open_story: Box<Fn(i32)>,
}

impl HomePage {
    pub fn new(on_open_story: Box<Fn(i32)>) -> HomePage {
        HomePage {
            stories: vec![],
            current_story: 0,
            on_open_story: on_open_story,
        }
    }
}

impl Page for HomePage {
    fn render(&self) {
        print_stories(self.stories.clone(), self.current_story as i32);
    }

    fn fetch_data(&mut self) {
        self.stories = get_stories("1");
    }

    fn input(&mut self, key: i32) {
        match key {
            KEY_DOWN => {
                if self.current_story < self.stories.len() {
                    self.current_story = self.current_story + 1;
                }
            }
            KEY_UP => {
                if self.current_story != 0 {
                    self.current_story = self.current_story - 1;
                }
            }
            // 10: enter
            10 => {
                let id = self.stories[self.current_story].id;
                (self.on_open_story)(id);
            }
            _ => {}
        }
    }
}
