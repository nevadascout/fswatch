extern crate notify;
extern crate timer;
extern crate chrono;
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

    let timer = timer::Timer::new();
    let guard: timer::Guard;

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(notify::Event { path: Some(path), op: Ok(op) }) => {
                println!("{:?} {:?}", op, path);
                // TODO -- find a way to handle large directory changes
                // (don't spawn a new rsync process for each file added/changed)
                // spawn a single process once all creates/copies are done

                // TODO -- work out how to check if this is instantiated
                if guard {
                    // Kill the guard and stop the timer
                    drop(guard);
                }

                // Debug
                println!("file watch event");

                guard = timer.schedule_with_delay(chrono::Duration::milliseconds(500), move || {
                    // Callback -- start the rsync process
                    // Command::new("sh").arg(command).spawn().expect("command failed!");
                });

                // NOTE: This function prevents the loop from looping
                // it waits until a file watch event is detected, then loops
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
