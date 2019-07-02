use ansi_term::Style;
use crate::models::Story;
use crate::models::StoryDetails;
use crate::models::User;

pub fn print_story(story: StoryDetails, indentation: usize) {
    let indent = " ".repeat(indentation);
    if story.title.is_some() {
        println!("{}{}", indent, story.title.unwrap());
    }
    println!("{}{}", indent, story.content);
    println!(
        "{}{} by {}",
        indent,
        story.time_ago,
        story.user.unwrap_or(String::from("unknown"))
    );
    for comment in story.comments {
        print_story(comment, indentation + 4);
    }
    println!();
}

pub fn print_stories(stories: Vec<Story>, active: i32) {
    let dimmed_style: Style = Style::new().dimmed();
    let underline_style: Style = Style::new().underline();

    for (index, story) in stories.iter().enumerate() {
        let item_index = if index < 9 {
            String::from(" ") + &(index + 1).to_string()
        } else {
            (index + 1).to_string()
        };

        if (index as i32) == active {
            println!(
                " {}) {}",
                item_index,
                underline_style.bold().paint(&story.title)
            );
        } else {
            println!(" {}) {}", item_index, &story.title);
        }

        println!(
            "       {}",
            match &story.url {
                Some(url) => dimmed_style.paint(url).to_string(),
                None => String::new(),
            }
        );

        print!("       ");
        print!(
            "{}",
            match &story.points {
                Some(points) => points.to_string() + " points",
                None => String::new(),
            }
        );
        print!(
            "{}",
            match &story.user {
                Some(user) => String::from(" by ") + &underline_style.paint(user),
                None => String::new(),
            }
        );
        print!(" {}", dimmed_style.paint(&story.time_ago));
        print!(" | {} comments", story.comments_count.to_string());
        println!("\n");
    }
}
