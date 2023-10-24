use egui::{Button, CentralPanel, Context, ScrollArea};

use crate::RcloneApp;

pub fn render_settings(ctx: &Context, app: &mut RcloneApp) {
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("Settings");

        ui.add_space(8.0);

        ScrollArea::new([false, true])
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Backup rclone config file:");
                    if ui.button("Backup").clicked() {
                        app.rclone.create_backup();
                    }
                });

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label("Restore rclone config file:");
                    if ui.add_enabled(false, Button::new("Restore")).clicked() {
                        // app.rclone.restore_backup();
                    }
                });

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label("Theme mode:");
                    let mut visuals = ui.ctx().style().visuals.clone();
                    visuals.light_dark_radio_buttons(ui);
                    ui.ctx().set_visuals(visuals);
                });

                ui.add_space(8.0);
            });
    });
}
