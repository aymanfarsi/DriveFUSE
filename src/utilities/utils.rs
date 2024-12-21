use std::{
    env,
    path::PathBuf,
    process::{Command, Stdio},
};

use auto_launch::AutoLaunchBuilder;
#[cfg(target_os = "windows")]
use directories::{BaseDirs, UserDirs};

use crate::DriveFUSE;

#[cfg(target_os = "windows")]
use {std::os::windows::process::CommandExt, winapi::um::winbase, windows::Win32};

#[cfg(target_os = "linux")]
use std::io::Cursor;

#[cfg(target_os = "linux")]
pub fn open_drive_location(name: String) {
    let username = whoami::username();
    let path = format!("/home/{}/drive_fuse/{}", username, name);

    let _ = Command::new("xdg-open")
        .arg(&path)
        .spawn()
        .expect("Unable to open drive location");
}

#[cfg(target_os = "linux")]
pub fn load_icon(bytes: &[u8]) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load(Cursor::new(bytes), image::ImageFormat::Png)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

#[cfg(target_family = "unix")]
pub fn check_if_mounted(_name: String) {
    let mut cmd = Command::new("df");
    cmd.args(["-hT", "|", "grep", "rclone"]);

    let process = cmd.output();
    match process {
        Ok(result) => {
            let output = String::from_utf8_lossy(&result.stdout);
            tracing::info!("{}", output);
        }
        Err(err) => {
            tracing::error!("{}", err);
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
            tracing::error!("Error while getting storage {} about info", name);
            Err(err.to_string())
        }
    }
}

#[cfg(target_os = "linux")]
pub fn unmount_delete_directory(name: String) {
    use std::{fs, path::Path};

    let username = whoami::username();
    let path = format!("/home/{}/drive_fuse/{}", username, name);

    let _ = Command::new("rm")
        .args(["-d", &path])
        .spawn()
        .expect("Unable to delete directory");

    fs::remove_dir(Path::new(&path)).expect("Unable to remove directory");
}

#[cfg(target_os = "macos")]
pub fn unmount_delete_directory(name: String) {
    use std::{fs, path::Path};

    let username = whoami::username();
    let path = format!("/Users/{}/drive_fuse/{}", username, name);

    let _ = Command::new("rm")
        .args(["-d", &path])
        .spawn()
        .expect("Unable to delete directory");

    fs::remove_dir(Path::new(&path)).expect("Unable to remove directory");
}

pub fn enable_auto_mount(app: &mut DriveFUSE) {
    app.app_config.set_is_auto_mount(true);
}

pub fn disable_auto_mount(app: &mut DriveFUSE) {
    app.app_config.set_is_auto_mount(false);
}

pub fn enable_auto_start_app() {
    let auto = AutoLaunchBuilder::new()
        .set_app_name("DriveFUSE")
        .set_app_path(
            env::current_exe()
                .expect("Unable to get current exe path")
                .to_str()
                .expect("Unable to convert path to string"),
        )
        .set_args(&["--minimized"])
        .build()
        .expect("Unable to build AutoLaunch");

    if !auto
        .is_enabled()
        .expect("Unable to check if app is enabled")
    {
        auto.enable().expect("Unable to enable app");
    }
}

pub fn is_app_auto_start() -> bool {
    let auto = AutoLaunchBuilder::new()
        .set_app_name("DriveFUSE")
        .set_app_path(
            env::current_exe()
                .expect("Unable to get current exe path")
                .to_str()
                .expect("Unable to convert path to string"),
        )
        .set_args(&["--minimized"])
        .build()
        .expect("Unable to build AutoLaunch");

    auto.is_enabled()
        .expect("Unable to check if app is enabled")
}

pub fn disable_auto_start_app() {
    let auto = AutoLaunchBuilder::new()
        .set_app_name("DriveFUSE")
        .set_app_path(
            env::current_exe()
                .expect("Unable to get current exe path")
                .to_str()
                .expect("Unable to convert path to string"),
        )
        .set_args(&["--minimized"])
        .build()
        .expect("Unable to build AutoLaunch");

    if auto
        .is_enabled()
        .expect("Unable to check if app is enabled")
    {
        auto.disable().expect("Unable to disable app");
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
    return UserDirs::new().map(|user_dirs| {
        user_dirs
            .document_dir()
            .expect("Unable to get document directory")
            .join("drive_fuse")
    });

    #[cfg(not(target_os = "windows"))]
    if cfg!(target_os = "linux") {
        Some(PathBuf::from(format!(
            "/home/{}/.config/drive_fuse",
            whoami::username()
        )))
    } else if cfg!(target_os = "macos") {
        Some(PathBuf::from(format!(
            "/Users/{}/.config/drive_fuse",
            whoami::username()
        )))
    } else {
        None
    }
}

// Google Drive
pub fn add_google_drive_storage(name: String) {
    tokio::spawn(async move {
        let mut cmd = Command::new("rclone");
        let cmd = cmd.args(&[
            String::from("config"),
            String::from("create"),
            name.trim().to_string(),
            String::from("drive"),
            String::from("config_is_local=true"),
        ]);

        #[cfg(target_os = "windows")]
        cmd.creation_flags(winbase::CREATE_NO_WINDOW);

        cmd.spawn().expect("Unable to spawn command");
    });
}

// OneDrive
pub fn add_onedrive_storage(name: String) {
    tokio::spawn(async move {
        let mut cmd = Command::new("rclone");
        let cmd = cmd.args(&[
            String::from("config"),
            String::from("create"),
            name.trim().to_string(),
            String::from("onedrive"),
            String::from("config_is_local=true"),
        ]);

        #[cfg(target_os = "windows")]
        cmd.creation_flags(winbase::CREATE_NO_WINDOW);

        cmd.spawn().expect("Unable to spawn command");
    });
}

// Dropbox
pub fn add_dropbox_storage(name: String) {
    tokio::spawn(async move {
        let mut cmd = Command::new("rclone");
        let cmd = cmd.args(&[
            String::from("config"),
            String::from("create"),
            name.trim().to_string(),
            String::from("dropbox"),
            String::from("config_is_local=true"),
        ]);

        #[cfg(target_os = "windows")]
        cmd.creation_flags(winbase::CREATE_NO_WINDOW);

        cmd.spawn().expect("Unable to spawn command");
    });
}

// Google Photos
pub fn add_google_photos_storage(name: String) {
    tokio::spawn(async move {
        let mut cmd = Command::new("rclone");
        let cmd = cmd.args(&[
            String::from("config"),
            String::from("create"),
            name.trim().to_string(),
            String::from("googlephotos"),
            String::from("config_is_local=true"),
        ]);

        #[cfg(target_os = "windows")]
        cmd.creation_flags(winbase::CREATE_NO_WINDOW);

        cmd.spawn().expect("Unable to spawn command");
    });
}

// Mega
pub fn add_mega_storage(name: String) {
    tokio::spawn(async move {
        let mut cmd = Command::new("rclone");
        let cmd = cmd.args(&[
            String::from("config"),
            String::from("create"),
            name.trim().to_string(),
            String::from("mega"),
            String::from("config_is_local=true"),
        ]);

        #[cfg(target_os = "windows")]
        cmd.creation_flags(winbase::CREATE_NO_WINDOW);

        cmd.spawn().expect("Unable to spawn command");
    });
}

// NextCloud
pub fn add_nextcloud_storage(name: String) {
    tokio::spawn(async move {
        let mut cmd = Command::new("rclone");
        let cmd = cmd.args(&[
            String::from("config"),
            String::from("create"),
            name.trim().to_string(),
            String::from("nextcloud"),
            String::from("config_is_local=true"),
        ]);

        #[cfg(target_os = "windows")]
        cmd.creation_flags(winbase::CREATE_NO_WINDOW);

        cmd.spawn().expect("Unable to spawn command");
    });
}
