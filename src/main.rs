extern crate notify;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::sync_channel;
use std::thread;
use notify::{RecommendedWatcher, Watcher};
use std::sync::mpsc::channel;

fn watch() -> notify::Result<()> {
    // Create a channel to recieve the events
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You must also access each implementation directly (eg. INotifyWatcher)
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx));

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    try!(watcher.watch("/Users/joe/dev/mover/test"));

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(notify::Event { path: Some(path), op: Ok(op) }) => {
                println!("{:?} {:?}", op, path);
            }
            Err(e) => println!("watch error {}", e),
            _ => (),
        }
    }
}

fn main() {
    if let Err(err) = watch() {
        println!("Error! {:?}", err)
    }
}