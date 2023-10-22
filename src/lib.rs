#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod utils {
    pub mod enums;
    pub mod tray_menu;
    pub mod utils;
}
pub use app::RcloneApp;
pub mod backend {
    pub mod rclone;
}
pub mod ui {
    pub mod main_page;
}
