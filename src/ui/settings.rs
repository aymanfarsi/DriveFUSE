use egui::{CentralPanel, Context};

use crate::RcloneApp;

pub fn render_settings(ctx: &Context, _app: &mut RcloneApp) {
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("Settings");
        ui.label("Settings");
    });
}
