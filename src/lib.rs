#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod utilities {
    pub mod enums;
    pub mod tray_menu;
    pub mod utils;
}
pub use app::DriveFUSE;
pub mod backend {
    pub mod app_config;
    pub mod mounting;
    pub mod mounting_options;
    pub mod rclone;
}
pub mod ui {
    pub mod manage;
    pub mod mount_unmount;
    pub mod settings;
    pub mod top_panel;
}
pub mod error_app;
