#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, fs::create_dir_all, path::Path, process::Command};

use eframe::icon_data::from_png_bytes;
use egui::{Vec2, ViewportBuilder};
use tokio::runtime::Runtime;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::time::ChronoLocal;

#[cfg(target_os = "windows")]
use {directories::UserDirs, std::os::windows::process::CommandExt, winapi::um::winbase};

use drive_fuse::{error_app::ErrorApp, DriveFUSE};

fn main() -> eframe::Result<()> {
    #[cfg(target_os = "windows")]
    let dir = UserDirs::new()
        .expect("Unable to get user directories")
        .document_dir()
        .expect("Unable to get document directory")
        .to_str()
        .expect("Unable to convert path to string")
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
        create_dir_all(&dir).expect("Unable to create directory");
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
        let min_size = Vec2::new(395., 292.5);
        let native_options = eframe::NativeOptions {
            centered: true,
            persist_window: true,
            viewport: ViewportBuilder::default()
                .with_decorations(true)
                .with_transparent(false)
                .with_resizable(true)
                .with_min_inner_size(min_size)
                .with_inner_size(min_size)
                .with_icon(
                    from_png_bytes(include_bytes!("../assets/drivefuse.png"))
                        .expect("Unable to get icon data"),
                ),
            ..Default::default()
        };
        eframe::run_native(
            "DriveFUSE",
            native_options,
            Box::new(move |_cc| Ok(Box::new(error_app))),
        )
    } else {
        let rt = Runtime::new().expect("Unable to create Runtime");
        let _enter = rt.enter();

        #[cfg(target_os = "linux")]
        {
            create_dir_all(format!("/home/{}/drive_fuse", whoami::username()))
                .expect("Unable to create directory");

            tokio::spawn(async move {
                use drive_fuse::utilities::{tray_menu::init_tray_menu, utils::load_icon};
                use tray_icon::{menu::Menu, TrayIconBuilder};

                let app_icon = load_icon(include_bytes!("../assets/drivefuse.png"));

                let mut menu = Menu::new();
                init_tray_menu(&mut menu);

                gtk::init().unwrap();
                let _tray_icon = TrayIconBuilder::new()
                    .with_menu(Box::new(menu))
                    .with_icon(app_icon)
                    .with_tooltip("DriveFUSE Tray")
                    .build()
                    .expect("Error creating tray icon");
                gtk::main();
            });
        }

        #[cfg(target_os = "macos")]
        create_dir_all(format!("/Users/{}/drive_fuse", whoami::username()))
            .expect("Unable to create directory");

        let min_size = Vec2::new(490., 292.5);
        let native_options = eframe::NativeOptions {
            centered: true,
            viewport: ViewportBuilder::default()
                .with_decorations(true)
                .with_transparent(false)
                .with_resizable(true)
                .with_min_inner_size(min_size)
                .with_inner_size(min_size)
                .with_icon(
                    from_png_bytes(include_bytes!("../assets/drivefuse.png"))
                        .expect("Unable to get icon data"),
                ),
            ..Default::default()
        };
        eframe::run_native(
            "DriveFUSE",
            native_options,
            Box::new(|_cc| Ok(Box::new(DriveFUSE::new()))),
        )
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

            let output = output.output().expect("Failed to execute command");
            if !output.status.success() {
                missing_dependencies.push("FUSE".to_string());
            }
        }
        "macos" => {
            // Check if FUSE is installed
            let mut cmd = Command::new("which");
            let output = cmd.arg("umount");

            let output = output.output().expect("Failed to execute command");
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

    let output = output.output();
    match output {
        Ok(output) => {
            if !output.status.success() {
                missing_dependencies.push("Rclone".to_string());
            }
        }
        Err(err) => {
            tracing::error!("Error in check_dependencies while checking rclone: {}", err);
            missing_dependencies.push("Rclone".to_string());
        }
    };

    missing_dependencies
}
