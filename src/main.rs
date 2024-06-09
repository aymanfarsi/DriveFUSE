#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, path::Path, process::Command};

use drive_fuse::{error_app::ErrorApp, RcloneApp};
use eframe::IconData;
use tokio::runtime::Runtime;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::time::ChronoLocal;

#[cfg(target_os = "windows")]
use {directories::UserDirs, std::os::windows::process::CommandExt, winapi::um::winbase};

use std::fs::create_dir_all;

fn main() -> eframe::Result<()> {
    #[cfg(target_os = "windows")]
    let dir = UserDirs::new()
        .unwrap()
        .document_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        + "/drive_fuse";

    #[cfg(not(target_os = "windows"))]
    let dir = format!(
        "/{}/{}/Documents/drive_fuse",
        if cfg!(target_os = "linux") {
            "home"
        } else {
            "Users"
        },
        whoami::username()
    );

    if !Path::new(&dir).exists() {
        create_dir_all(&dir).unwrap();
    }

    let file_appender = tracing_appender::rolling::never(dir, "drive_fuse.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let timer = ChronoLocal::new("%m/%d/%YT%H:%M:%S".to_owned());

    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_level(true)
        .with_file(true)
        .with_target(true)
        .with_line_number(true)
        .with_timer(timer)
        .with_writer(std::io::stderr)
        .with_writer(non_blocking)
        .with_max_level(LevelFilter::WARN)
        .init();

    match machine_uid::get() {
        Ok(machine_id) => tracing::info!("Machine ID is: {}", machine_id),
        Err(err) => tracing::error!("Error in main while getting machine id: {}", err),
    };

    let platform = env::consts::OS;
    let missing_dependencies = check_dependencies(platform);
    let is_platform_supported = platform == "windows" || platform == "linux" || platform == "macos";
    if !missing_dependencies.is_empty() || !is_platform_supported {
        let error_app = ErrorApp {
            is_platform_supported,
            platform: platform.to_string(),
            missing_dependencies,
        };
        let native_options = eframe::NativeOptions {
            centered: true,
            decorated: true,
            transparent: false,
            resizable: true,
            min_window_size: Some(egui::Vec2::new(395., 292.5)),
            initial_window_size: Some(egui::Vec2::new(395., 292.5)),
            icon_data: Some(
                IconData::try_from_png_bytes(include_bytes!("../assets/drivefuse.png")).unwrap(),
            ),
            ..Default::default()
        };
        eframe::run_native(
            "DriveFUSE",
            native_options,
            Box::new(move |_cc| Box::new(error_app)),
        )
    } else {
        #[cfg(target_os = "linux")]
        create_dir_all(format!("/home/{}/drive_fuse", whoami::username())).unwrap();

        #[cfg(target_os = "macos")]
        create_dir_all(format!("/Users/{}/drive_fuse", whoami::username())).unwrap();

        let rt = Runtime::new().expect("Unable to create Runtime");
        let _enter = rt.enter();

        let app = RcloneApp::default();
        #[cfg(target_os = "windows")]
        let min_size = egui::Vec2::new(490., 292.5);
        #[cfg(not(target_os = "windows"))]
        let min_size = egui::Vec2::new(395., 292.5);

        let native_options = eframe::NativeOptions {
            centered: true,
            decorated: true,
            transparent: false,
            resizable: true,
            min_window_size: Some(min_size),
            initial_window_size: Some(min_size),
            icon_data: Some(
                IconData::try_from_png_bytes(include_bytes!("../assets/drivefuse.png")).unwrap(),
            ),
            ..Default::default()
        };
        eframe::run_native("DriveFUSE", native_options, Box::new(|_cc| Box::new(app)))
    }
}

fn check_dependencies(platform: &str) -> Vec<String> {
    let mut missing_dependencies = Vec::new();

    match platform {
        "windows" => {
            // Check if WinFsp is installed
            if !Path::new("C:/Program Files (x86)/WinFsp").exists() {
                missing_dependencies.push("WinFsp".to_string());
            }
        }
        "linux" => {
            // Check if FUSE is installed
            let mut cmd = Command::new("which");
            let output = cmd.arg("fusermount");

            let output = output.output().unwrap();
            if !output.status.success() {
                missing_dependencies.push("FUSE".to_string());
            }
        }
        "macos" => {
            // Check if FUSE is installed
            let mut cmd = Command::new("which");
            let output = cmd.arg("umount");

            let output = output.output().unwrap();
            if !output.status.success() {
                missing_dependencies.push("FUSE".to_string());
            }
        }
        _ => {}
    }

    // Check if Rclone is installed
    let mut cmd = Command::new("rclone");
    let output = cmd.arg("--version");

    #[cfg(target_os = "windows")]
    output.creation_flags(winbase::CREATE_NO_WINDOW);

    let output = output.output().unwrap();
    if !output.status.success() {
        missing_dependencies.push("Rclone".to_string());
    }

    missing_dependencies
}
