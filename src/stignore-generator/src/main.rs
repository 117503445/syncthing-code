use notify::Config;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::env;
use std::path::Path;

use env_logger::{Builder, Env};
use log::info;
use stignore_generator::write_stgitignore;
use stignore_generator::write_stignore;

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

    // write();

    if let Err(error) = watch(workspace_path) {
        log::error!("Error: {error:?}");
    }
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                for path in event.paths {
                    if path.display().to_string().ends_with(".gitignore") {
                        write();
                    }
                }
            }
            Err(error) => log::error!("Error: {error:?}"),
        }
    }

    Ok(())
}
