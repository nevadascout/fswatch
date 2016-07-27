extern crate notify;
use notify::{RecommendedWatcher, Watcher};
use std::sync::mpsc::channel;
use std::env;
use std::string::String;

fn watch() -> notify::Result<()> {
    // let args: Vec<String> = env::args().map(|s| s.into_string().unwrap()).collect();

    // if args.len() > 2 {
    //     if args[1] == "-c" {
    //         // Check if args[2] is a valid file path
    //     } else {
    //         println!("Please use ./mover -c /path/to/config.json");
    //     }
    // }
    
    // Hardcode config file location temporarily
    // "~/mover_config.json"

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
