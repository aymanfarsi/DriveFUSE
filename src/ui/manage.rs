use egui::{CentralPanel, Context};

use crate::RcloneApp;

pub fn render_manage(ctx: &Context, _app: &mut RcloneApp) {
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("Manage");
        ui.label("Manage");
    });
}
