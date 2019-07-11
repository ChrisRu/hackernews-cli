use crate::models::Story;
use crate::models::StoryDetails;
use html2text::from_read;
use ncurses::*;

const COMMENT_INDENTATION_SIZE: usize = 4;
const COMMENT_WRAP: usize = 80;

fn html2text(content: &str, indent: &str) -> String {
    from_read(content.as_bytes(), COMMENT_WRAP).replace("\n", &["\n", &indent].join(""))
}

fn print_story_indented(story: StoryDetails, current_indentation_size: usize) {
    let indent = " ".repeat(current_indentation_size);

    // Print title
    if story.title.is_some() {
        attron(A_BOLD());
        addstr(&format!("{}{}\n", &indent, &story.title.unwrap()));
        attroff(A_BOLD());
    }

    // Print link
    if story.r#type == "link" {
        addstr(&format!("{}{}\n", &indent, &story.url.unwrap()));
    }

    // Print content
    addstr(&format!(
        "{}{}",
        &indent,
        &html2text(&story.content, &indent)
    ));

    // Print comment details
    if story.points.is_some() {
        addstr(&story.points.unwrap().to_string());
        attron(A_DIM());
        addstr(" ▴");
    } else {
        attron(A_DIM());
    }
    addstr(&format!("{} by ", &story.time_ago));
    attroff(A_DIM());
    attron(A_UNDERLINE());
    addstr(&story.user.unwrap_or(String::from("unknown")));
    attroff(A_UNDERLINE());
    addstr("\n\n");

    for comment in story.comments {
        print_story_indented(comment, current_indentation_size + COMMENT_INDENTATION_SIZE);
    }

    addstr("\n");
}

pub fn print_story(story: StoryDetails) {
    print_story_indented(story, 1);
}

pub fn print_stories(stories: Vec<Story>, active: i32) {
    for (index, story) in stories.iter().enumerate() {
        let item_index = if index < 9 {
            String::from(" ") + &(index + 1).to_string()
        } else {
            (index + 1).to_string()
        };

        // Print title
        addstr(" ");
        addstr(&item_index.to_string());
        addstr(") ");
        if (index as i32) == active {
            attron(A_UNDERLINE());
        }
        addstr(&story.title);
        attroff(A_UNDERLINE());
        addstr("\n");

        // Print URL
        if story.url.is_some() {
            addstr("       ");
            attron(A_DIM());
            addstr(&story.url.clone().unwrap());
            attroff(A_DIM());
            addstr("\n");
        }

        // Print details
        addstr("       ");
        addstr(" ");
        if story.points.is_some() {
            addstr(&story.points.unwrap().to_string());
            attron(A_DIM());
            addstr(" ▴");
            attroff(A_DIM());
        }
        addstr(" |");
        if story.user.is_some() {
            addstr(" by ");
            attron(A_UNDERLINE());
            addstr(&story.user.clone().unwrap());
            attroff(A_UNDERLINE());
        }

        attron(A_DIM());
        addstr(" ");
        addstr(&story.time_ago);
        attroff(A_DIM());

        addstr(" | ");
        addstr(&story.comments_count.to_string());
        addstr(" comments");
        addstr("\n");
    }
}
