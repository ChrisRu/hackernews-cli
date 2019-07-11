use ncurses::*;

use keys::is_exit;
use pages::commentpage::*;
use pages::homepage::*;
use pages::Page;

use std::thread;


mod fetch;
mod keys;
mod pages;
mod print;
mod models;
mod spinner;


fn render(page: Box<&mut impl Page>) -> bool {
    let spinner_handle = spinner::create_spinner_thread();
    page.fetch_data();
    spinner_handle.stop().join().unwrap();
    clear();
    page.render();
    refresh();

    loop {
        let key = getch();
        if is_exit(key) {
            return true;
        }

        page.input(key);
        clear();
        page.render();
        refresh();
    }
}

fn main() {
    setlocale(LcCategory::all, "en_US.UTF-8");
    initscr();
    raw();
    noecho();
    keypad(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    thread::spawn(move || {
        fn on_close() {
            endwin();
        }

        fn open_comments(id: i32) {
            loop {
                if render(Box::new(&mut CommentPage::new(id, Box::new(on_close)))) {
                    break;
                }
            }
        }

        loop {
            if render(Box::new(&mut HomePage::new(Box::new(open_comments)))) {
                break;
            }
        }

        endwin();
    })
    .join()
    .unwrap();
}
