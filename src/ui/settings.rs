use egui::{Button, CentralPanel, Context, ScrollArea};

use crate::{utilities::utils::is_app_auto_start, RcloneApp};

pub fn render_settings(ctx: &Context, app: &mut RcloneApp) {
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("Settings");

        ui.add_space(8.0);

        ScrollArea::new([false, true])
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                // add expandable section
                ui.collapsing("Config file", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Backup rclone config file:");
                        if ui.button("Backup").clicked() {
                            app.rclone.create_backup();
                        }
                    });

                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        ui.label("Restore rclone config file:");
                        if ui.add(Button::new("Restore")).clicked() {
                            // app.rclone.restore_backup();
                        }
                    });
                });

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label("Theme mode:");
                    let mut visuals = ui.ctx().style().visuals.clone();
                    visuals.light_dark_radio_buttons(ui);
                    ui.ctx().set_visuals(visuals);
                });

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label("Auto start:");
                    let is_auto_start = is_app_auto_start();
                    if ui
                        .add(Button::new(if is_auto_start {
                            "Disable"
                        } else {
                            "Enable"
                        }))
                        .clicked()
                    {
                        if is_auto_start {
                            disable_auto_start_app();
                        } else {
                            enable_auto_start_app();
                        }
                    }
                });
            });
    });
}
