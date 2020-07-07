use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn run_listener() {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    watcher.watch("/home/flo/dev/ascii-tool/tests", RecursiveMode::Recursive);

    let mut counter = 0;

    loop {
        match rx.recv() {
            Ok(event) => {
                println!("{:?}", event);
                println!("{}", counter);
                counter = counter + 1;
            }
            Err(er) => println!("error: {:?}", er),
        }
    }
}

// pub fn add_file(file: &File) {}

// pub fn remove_file(file: &File) {}
