use ncurses::{clear,addstr,refresh};

use std::thread::{sleep};
use std::time::*;
use stoppable_thread::*;

pub fn create_spinner_thread() -> StoppableHandle<()> {
    spawn(|stopped| {
        let mut cycle = ['⠂', '-', '–', '—', '–', '-'].iter().cycle();

        while !stopped.get() {
            let character = cycle.next().unwrap().to_string();
            clear();
            addstr(&format!("\n {}", &character));
            refresh();
            sleep(Duration::from_millis(100));
        }
    })
}
