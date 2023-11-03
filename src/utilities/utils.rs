use auto_launch::AutoLaunchBuilder;
use directories::{BaseDirs, UserDirs};
use std::{os::windows::process::CommandExt, path::PathBuf, process::Command, env};
use winapi::um::winbase;
use windows::Win32;

pub fn enable_auto_start_app() {
    let auto = AutoLaunchBuilder::new()
        .set_app_name("RcloneApp")
        .set_app_path(env::current_exe().unwrap().to_str().unwrap())
        .set_args(&["--minimized"])
        .build()
        .unwrap();
    if !auto.is_enabled().unwrap() {
        auto.enable().unwrap();
    }
}

pub fn is_app_auto_start() -> bool {
    let auto = AutoLaunchBuilder::new()
        .set_app_name("RcloneApp")
        .set_app_path(env::current_exe().unwrap().to_str().unwrap())
        .set_args(&["--minimized"])
        .build()
        .unwrap();
    auto.is_enabled().unwrap()
}

pub fn disable_auto_start_app() {
    let auto = AutoLaunchBuilder::new()
        .set_app_name("RcloneApp")
        .set_app_path(env::current_exe().unwrap().to_str().unwrap())
        .set_args(&["--minimized"])
        .build()
        .unwrap();
    if auto.is_enabled().unwrap() {
        auto.disable().unwrap();
    }
}

pub fn available_drives() -> Vec<char> {
    let drive_letters = unsafe { Win32::Storage::FileSystem::GetLogicalDrives() };
    let mut available_drives = vec![];
    for i in 0..26 {
        if drive_letters & (1 << i) == 0 {
            available_drives.push((i + 65) as u8 as char);
        }
    }
    available_drives
}

pub fn rclone_config_path() -> Option<PathBuf> {
    BaseDirs::new().map(|base_dirs| base_dirs.config_dir().join("rclone"))
}

pub fn app_config_path() -> Option<PathBuf> {
    UserDirs::new().map(|user_dirs| user_dirs.document_dir().unwrap().join("rclone_app"))
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
            .creation_flags(winbase::CREATE_NO_WINDOW)
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
            .creation_flags(winbase::CREATE_NO_WINDOW)
            .spawn()
            .unwrap();
    });
}
