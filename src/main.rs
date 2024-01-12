#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    env,
    path::Path,
    process::{exit, Command},
};

use drive_af::RcloneApp;
use eframe::IconData;
use std::io;
use tokio::runtime::Runtime;
use tracing_subscriber::{fmt, subscribe::CollectExt, EnvFilter};

#[cfg(target_os = "windows")]
use {std::os::windows::process::CommandExt, winapi::um::winbase};

use std::fs::create_dir_all;

fn main() {
    let username = whoami::username();

    let dir = format!("/home/{}/Documents/DriveAF/logs", username.clone());
    if !Path::new(&dir).exists() {
        create_dir_all(&dir).unwrap();
    }

    let file_appender = tracing_appender::rolling::never(dir, "logs.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let collector = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()))
        .with(fmt::Subscriber::new().with_writer(io::stdout))
        .with(fmt::Subscriber::new().with_writer(non_blocking));
    tracing::collect::set_global_default(collector).expect("Unable to set a global collector");

    let machine_id = machine_uid::get().unwrap();
    println!("{}", machine_id);

    let platform = env::consts::OS;
    let missing_dependencies = check_dependencies(platform);
    if !missing_dependencies.is_empty() {
        println!("Missing dependencies: {}", missing_dependencies.join(", "));
        println!("Please install them and try again!");
        exit(1);
    }

    if platform == "windows" || platform == "linux" {
        #[cfg(target_os = "linux")]
        create_dir_all(format!("/home/{}/drive_af", username)).unwrap();

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
            icon_data: Some(
                IconData::try_from_png_bytes(include_bytes!("../assets/DriveAF-nobg.png")).unwrap(),
            ),
            ..Default::default()
        };
        let _ = eframe::run_native("DriveAF", native_options, Box::new(|_cc| Box::new(app)));
    } else {
        println!("This app only supports Windows and Linux FOR NOW!");
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
            let mut cmd = Command::new("which");
            let output = cmd.arg("fusermount");

            let output = output.output().unwrap();
            if !output.status.success() {
                missing_dependencies.push("FUSE".to_string());
            }
        }
        "macos" => {
            // Check if FUSE is installed
            let mut cmd = Command::new("pkg-config");
            let output = cmd.arg("--exists").arg("fuse");

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
