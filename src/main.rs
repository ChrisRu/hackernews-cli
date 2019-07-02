use std::io::{self, Write};
use std::{thread, time};
use stoppable_thread;

mod fetch;

mod models;
mod print;
fn main() {
    let loop_handle = stoppable_thread::spawn(|stopped| {
        let mut stdout = io::stdout();
        let mut cycle = ["⠂", "-", "–", "—", "–", "-"].iter().cycle();

        while !stopped.get() {
            stdout.write(cycle.next().unwrap().as_bytes()).unwrap();
            stdout.flush().unwrap();
            thread::sleep(time::Duration::from_millis(100));
            stdout.write("\x08".as_bytes()).unwrap();
        }
    });

    thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(2000));

        let stories = fetch::get_stories("1");
        loop_handle.stop().join().unwrap();
        let (first_stories, _) = stories.split_at(10);
        print::print_stories(first_stories.to_vec(), -1);
    })
    .join()
    .unwrap();
}
