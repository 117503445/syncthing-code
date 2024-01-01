use std::env;
use std::path::Path;

use env_logger::{Builder, Env};
use log::info;
use stignore_generator::write_stgitignore;
use stignore_generator::write_stignore;

extern crate inotify;
use inotify::{Inotify, WatchMask};

fn write() {
    let workspace_dir: &Path = Path::new(".");

    let stignore_file = workspace_dir.join(".stignore");
    let stignore_file = stignore_file.as_path();

    write_stignore(stignore_file);
    write_stgitignore(workspace_dir);
}

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug")).init();
    info!("stignore-generator started");

    let workspace_path = Path::new("/workspace");

    env::set_current_dir(workspace_path).unwrap();
    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    inotify
        .watches()
        .add(
            workspace_path,
            WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE,
        )
        .expect("Failed to add inotify watch");

    write();
        
    let mut buffer = [0u8; 4096];
    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");

        for event in events {
            if event.name.is_some() {
                if event.name.unwrap() == ".gitignore" {
                    write();
                }
            }
        }
    }
}
