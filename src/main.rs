use std::{path::Path, sync::mpsc::channel, time::Duration};

use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::new_debouncer;

fn main() {
    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(
        Duration::from_secs(100),
        Some(Duration::from_millis(100)),
        move |event| {
            tx.send(event).unwrap();
            println!("Event sent");
        },
    )
    .unwrap();
    let watcher = debouncer.watcher();
    watcher
        .watch(Path::new("."), RecursiveMode::Recursive)
        .unwrap();
    rx.iter().for_each(|event| match event {
        Ok(event) => {
            println!("{:?}", event);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    });
}
