use std::{
    env,
    path::PathBuf,
    process::{Command, Stdio},
};

use auto_launch::AutoLaunchBuilder;
#[cfg(target_os = "windows")]
use directories::{BaseDirs, UserDirs};

#[cfg(target_os = "windows")]
use {std::os::windows::process::CommandExt, winapi::um::winbase, windows::Win32};

use crate::RcloneApp;

#[cfg(target_family = "unix")]
pub fn check_if_mounted(_name: String) {
    let mut cmd = Command::new("df");
    cmd.args(["-hT", "|", "grep", "rclone"]);

    let process = cmd.output();
    match process {
        Ok(result) => {
            let output = String::from_utf8_lossy(&result.stdout);
            println!("{}", output);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

pub fn get_info(name: String) -> Result<String, String> {
    let mut cmd = Command::new("rclone");
    cmd.args(["about", &format!("{}:", name), "--json"])
        .stdout(Stdio::piped());

    #[cfg(target_os = "windows")]
    cmd.creation_flags(winbase::CREATE_NO_WINDOW);

    let process = cmd.output();
    match process {
        Ok(result) => {
            let output = String::from_utf8_lossy(&result.stdout);
            Ok(output.to_string())
        }
        Err(err) => {
            eprintln!("Error while getting storage {} about info", name);
            Err(err.to_string())
        }
    }
}

#[cfg(target_os = "linux")]
pub fn unmount_delete_directory(name: String) {
    use std::{fs, path::Path};

    let username = whoami::username();
    let path = format!("/home/{}/drive_af/{}", username, name);

    let _ = Command::new("rm").args(["-d", &path]).spawn().unwrap();

    fs::remove_dir(Path::new(&path)).unwrap();
}

#[cfg(target_os = "macos")]
pub fn unmount_delete_directory(name: String) {
    use std::{fs, path::Path};

    let username = whoami::username();
    let path = format!("/Users/{}/drive_af/{}", username, name);

    let _ = Command::new("rm").args(["-d", &path]).spawn().unwrap();

    fs::remove_dir(Path::new(&path)).unwrap();
}

pub fn enable_auto_mount(app: &mut RcloneApp) {
    app.app_config.is_auto_mount = true;
    app.app_config.save();
}

pub fn disable_auto_mount(app: &mut RcloneApp) {
    app.app_config.is_auto_mount = false;
    app.app_config.save();
}

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

#[cfg(target_os = "windows")]
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
    #[cfg(target_os = "windows")]
    {
        let path = BaseDirs::new().map(|base_dirs| base_dirs.config_dir().join("rclone"));
        path
    }
    #[cfg(not(target_os = "windows"))]
    {
        Some(PathBuf::from(format!(
            "/{}/{}/.config/rclone",
            if cfg!(target_os = "linux") {
                "home"
            } else {
                "Users"
            },
            whoami::username()
        )))
    }
}

pub fn app_config_path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    return UserDirs::new().map(|user_dirs| user_dirs.document_dir().unwrap().join("rclone_app"));
    #[cfg(not(target_os = "windows"))]
    if cfg!(target_os = "linux") {
        Some(PathBuf::from(format!(
            "/home/{}/.config/rclone_app",
            whoami::username()
        )))
    } else if cfg!(target_os = "macos") {
        Some(PathBuf::from(format!(
            "/Users/{}/.config/rclone_app",
            whoami::username()
        )))
    } else {
        None
    }
}

pub fn add_google_drive_storage(name: String) {
    tokio::spawn(async move {
        let mut cmd = Command::new("rclone");
        let cmd = cmd.args(&[
            String::from("config"),
            String::from("create"),
            name.trim().to_string(),
            String::from("drive"),
            String::from("config_is_local"),
            String::from("true"),
        ]);

        #[cfg(target_os = "windows")]
        cmd.creation_flags(winbase::CREATE_NO_WINDOW);

        cmd.spawn().unwrap();
    });
}

pub fn add_onedrive_storage(name: String) {
    tokio::spawn(async move {
        let mut cmd = Command::new("rclone");
        let cmd = cmd.args(&[
            String::from("config"),
            String::from("create"),
            name.trim().to_string(),
            String::from("onedrive"),
            String::from("config_is_local"),
            String::from("true"),
        ]);

        #[cfg(target_os = "windows")]
        cmd.creation_flags(winbase::CREATE_NO_WINDOW);

        cmd.spawn().unwrap();
    });
}
