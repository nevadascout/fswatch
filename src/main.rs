extern crate notify;
use notify::{RecommendedWatcher, Watcher};
use std::sync::mpsc::channel;
use std::env;
use std::path::Path;
use std::process::Command;

fn watch() -> notify::Result<()> {

    // Take two args:
    // 1. file/folder to watch
    // 2. bash script to execute

    // Display usage message if num args != 3
    if env::args().count() != 3 {
        println!("Invalid usage. Example usage: ./fswatch /path/to/watch /command/to/run.sh");
        std::process::exit(1);
    }

    let args: Vec<String> = env::args().collect();

    // Path to watch
    let ref path = args[1];
    // Check the supplied path exists
    if !Path::new(path).exists() {
        println!("The supplied path '{}' does not exist!", path);
        std::process::exit(0);
    }

    // Get the command to run
    let ref command = args[2];
    // Check the supplied command exists and is a bash file
    if !Path::new(command).exists() {
        println!("The supplied bash command file '{}' does not exist!", path);
        std::process::exit(0);
    }

    // Create a channel to recieve the events
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You must also access each implementation directly (eg. INotifyWatcher)
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx));

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    try!(watcher.watch(path));
    println!("Starting file watcher on path '{}'...", path);

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(notify::Event { path: Some(path), op: Ok(op) }) => {
                println!("{:?} {:?}", op, path);
                // TODO -- don't run if something last file change was within 250ms

                // TODO -- wait 100ms before running to check for another change

                Command::new("sh").arg(command).spawn().expect("command failed!");
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
