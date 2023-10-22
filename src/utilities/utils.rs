use directories::BaseDirs;
use std::{path::PathBuf, process::Command};

pub fn rclone_config_path() -> Option<PathBuf> {
    BaseDirs::new().map(|base_dirs| base_dirs.config_dir().join("rclone"))
}

pub fn add_google_drive_storage(name: String) {
    tokio::spawn(async move {
        Command::new("rclone")
            .args(&[
                String::from("config"),
                String::from("create"),
                name.trim().to_string(),
                String::from("drive"),
                String::from("config_is_local"),
                String::from("true"),
            ])
            .spawn()
            .unwrap();
    });
}

pub fn add_onedrive_storage(name: String) {
    tokio::spawn(async move {
        Command::new("rclone")
            .args(&[
                String::from("config"),
                String::from("create"),
                name.trim().to_string(),
                String::from("onedrive"),
                String::from("config_is_local"),
                String::from("true"),
            ])
            .spawn()
            .unwrap();
    });
}
