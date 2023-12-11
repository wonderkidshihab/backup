use std::env;
use std::fs::{self, *};
use std::{path::Path, sync::mpsc::channel, time::Duration};

use notify::{EventKind, RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebouncedEvent};

fn main() {
    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(Duration::from_secs(1), None, move |event| {
        tx.send(event).unwrap();
        println!("Event sent");
    })
    .unwrap();
    let watcher = debouncer.watcher();
    watcher
        .watch(Path::new("some"), RecursiveMode::Recursive)
        .unwrap();
    rx.iter().for_each(|event| match event {
        Ok(result) => handle_event(result),
        Err(e) => println!("watch error: {:?}", e),
    });
}

const DIST_PATH: &str = "dist";

fn handle_event(result: Vec<DebouncedEvent>) {
    let current_working_directory = env::current_dir()
        .expect("Unable to get current working directory")
        .to_str()
        .unwrap()
        .to_string();
    for event in result {
        match event.event.kind {
            EventKind::Create(_) => {
                println!("Create event {}", event.paths[0].to_str().unwrap());
                let path = event.paths[0].to_str().unwrap();
                let dest_path = format!(
                    "{}/{}",
                    DIST_PATH,
                    path.trim_start_matches(&current_working_directory)
                );

                if fs::metadata(&dest_path).is_ok() {
                    fs::copy(path, &dest_path).unwrap();
                } else {
                    fs::create_dir_all(Path::new(&dest_path).parent().unwrap())
                        .expect("Unable to create directory");
                    fs::copy(path, &dest_path).expect("Unable to copy file");
                }
                println!("Create: {:?}", event);
            }
            EventKind::Modify(_) => {
                let path = event.paths[0].to_str().unwrap();
                let dest_path = format!(
                    "{}/{}",
                    DIST_PATH,
                    path.trim_start_matches(&current_working_directory)
                );
                if fs::metadata(&dest_path).is_ok() {
                    fs::copy(path, &dest_path).unwrap();
                } else {
                    fs::create_dir_all(Path::new(&dest_path).parent().unwrap()).unwrap();
                    fs::copy(path, &dest_path).unwrap();
                }
                println!("Modify: {:?}", event);
            }
            EventKind::Remove(_) => {
                let path = event.paths[0].to_str().unwrap();
                let dest_path = format!(
                    "{}/{}",
                    DIST_PATH,
                    path.trim_start_matches(&current_working_directory)
                );
                let result = fs::remove_file(&dest_path).unwrap();
                match result {
                    _ => {}
                }

                println!("Remove: {:?}", event);
            }
            EventKind::Any => {
                println!("Any: {:?}", event);
            }
            EventKind::Other => {
                println!("Other: {:?}", event);
            }
            EventKind::Access(_) => {
                println!("Access: {:?}", event);
            }
        }
    }
}
