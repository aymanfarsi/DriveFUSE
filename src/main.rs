#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    path::Path,
    process::{exit, Command},
};

use rclone_app::RcloneApp;
use tokio::runtime::Runtime;

fn main() {
    let platform = std::env::consts::OS;
    if platform == "windows" {
        let missing_dependencies = check_dependencies_windows();
        if !missing_dependencies.is_empty() {
            println!("Missing dependencies: {:?}", missing_dependencies);
            println!("Please install them and try again!");
            exit(1);
        }
        let rt = Runtime::new().expect("Unable to create Runtime");
        let _enter = rt.enter();

        let app = RcloneApp::default();

        let native_options = eframe::NativeOptions {
            centered: true,
            decorated: true,
            transparent: false,
            resizable: false,
            min_window_size: Some(egui::Vec2::new(400.0, 250.0)),
            initial_window_size: Some(egui::Vec2::new(400.0, 200.0)),
            ..Default::default()
        };
        let _ = eframe::run_native("Rclone App", native_options, Box::new(|_cc| Box::new(app)));
    } else {
        println!("This app only supports Windows FOR NOW!");
        println!("Your platform is: {}", platform);
    }
}

fn check_dependencies_windows() -> Vec<String> {
    let mut missing_dependencies = Vec::new();

    // Chek if WinFsp is installed
    if !Path::new("C:/Program Files (x86)/WinFsp").exists() {
        missing_dependencies.push("WinFsp".to_string());
    }

    // Check if Rclone is installed
    let output = Command::new("rclone").arg("--version").output().unwrap();
    if !output.status.success() {
        missing_dependencies.push("Rclone".to_string());
    }

    missing_dependencies
}
