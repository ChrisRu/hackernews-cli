
use keys::is_exit;
use ncurses::*;
use pages::homepage::*;
use pages::commentpage::*;
use pages::Page;

mod fetch;
mod keys;
mod print;
mod models;
mod spinner;
mod pages;

fn render(page: Box<&mut impl Page>) {
    let spinner_handle = spinner::create_spinner_thread();
    page.fetch_data();
    spinner_handle.stop().join().unwrap();

    clear();
    page.render();
    refresh();

    loop {
        let key = getch();
        if is_exit(key) {
            break;
        }
        page.input(key);

        clear();
        page.render();
        refresh();
    }
}

fn on_close() {
    endwin();
}

fn open_comments(id: i32) {
    render(Box::new(&mut CommentPage::new(id, Box::new(on_close))));
}

fn open_homepage() {
    render(Box::new(&mut HomePage::new(Box::new(open_comments))));
}

fn main() {
    setlocale(LcCategory::all, "en_US.UTF-8");
    initscr();
    raw();
    noecho();
    keypad(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    open_homepage();

    endwin();
}
