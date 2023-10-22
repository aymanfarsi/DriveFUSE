use rclone_app::RcloneApp;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().expect("Unable to create Runtime");
    let _enter = rt.enter();

    let app = RcloneApp::default();
    let native_options = eframe::NativeOptions {
        centered: true,
        initial_window_size: Some(egui::Vec2::new(400.0, 200.0)),
        ..Default::default()
    };
    let _ = eframe::run_native("Rclone App", native_options, Box::new(|_cc| Box::new(app)));
}
