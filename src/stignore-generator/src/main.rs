use std::env;
use std::path::Path;

use env_logger::{Builder, Env};
use log::info;
use stignore_generator::write_stgitignore;
use stignore_generator::write_stignore;

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug")).init();
    info!("start stignore-generator");

    env::set_current_dir(Path::new("/workspace")).unwrap();
    let workspace_dir: &Path = Path::new(".");

    let stignore_file = workspace_dir.join(".stignore");
    let stignore_file = stignore_file.as_path();
    
    write_stignore(stignore_file);
    write_stgitignore(workspace_dir);
}
