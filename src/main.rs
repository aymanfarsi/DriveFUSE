#![windows_subsystem = "windows"]

use std::{
    env,
    os::windows::process::CommandExt,
    path::Path,
    process::{exit, Command},
};

use rclone_app::RcloneApp;
use tokio::runtime::Runtime;
use winapi::um::winbase;

fn main() {
    let platform = env::consts::OS;
    let missing_dependencies = check_dependencies(platform);
    if !missing_dependencies.is_empty() {
        println!("Missing dependencies: {}", missing_dependencies.join(", "));
        println!("Please install them and try again!");
        exit(1);
    }

    if platform == "windows" {
        let rt = Runtime::new().expect("Unable to create Runtime");
        let _enter = rt.enter();

        let app = RcloneApp::default();

        let native_options = eframe::NativeOptions {
            centered: true,
            decorated: true,
            transparent: false,
            resizable: false,
            min_window_size: Some(egui::Vec2::new(430.0, 250.0)),
            initial_window_size: Some(egui::Vec2::new(430.0, 250.0)),
            ..Default::default()
        };
        let _ = eframe::run_native("Rclone App", native_options, Box::new(|_cc| Box::new(app)));
    } else {
        println!("This app only supports Windows FOR NOW!");
        println!("Your platform is: {}", platform);
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
            let output = Command::new("pkg-config")
                .arg("--exists")
                .arg("fuse")
                .creation_flags(winbase::CREATE_NO_WINDOW)
                .output()
                .unwrap();
            if !output.status.success() {
                missing_dependencies.push("FUSE".to_string());
            }
        }
        "macos" => {
            // Check if FUSE is installed
            let output = Command::new("pkg-config")
                .arg("--exists")
                .arg("fuse")
                .creation_flags(winbase::CREATE_NO_WINDOW)
                .output()
                .unwrap();
            if !output.status.success() {
                missing_dependencies.push("FUSE".to_string());
            }
        }
        _ => {}
    }

    // Check if Rclone is installed
    let output = Command::new("rclone")
        .arg("--version")
        .creation_flags(winbase::CREATE_NO_WINDOW)
        .output()
        .unwrap();
    if !output.status.success() {
        missing_dependencies.push("Rclone".to_string());
    }

    missing_dependencies
}
