#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod utils {
    pub mod enums;
    pub mod tray_menu;
}
pub use app::RcloneApp;
