use egui::{Button, CentralPanel, Context, ScrollArea};

use crate::{
    utilities::{
        enums::AppTheme,
        utils::{
            disable_auto_mount, disable_auto_start_app, enable_auto_mount, enable_auto_start_app,
            is_app_auto_start,
        },
    },
    RcloneApp,
};

pub fn render_settings(ctx: &Context, app: &mut RcloneApp) {
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("Settings");

        ui.add_space(8.0);

        ScrollArea::new([false, true])
            .auto_shrink([false; 2])
            .show(ui, |ui| {
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

                    for theme in AppTheme::values() {
                        let response = ui.selectable_value(
                            &mut app.app_config.current_theme,
                            theme,
                            theme.name().to_string(),
                        );
                        if response.clicked() {
                            theme.set_theme(ctx);
                            app.app_config.set_current_theme(theme);
                        }
                    }
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

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label("Auto mount:");
                    let is_auto_mount = app.app_config.is_auto_mount;
                    if ui
                        .add(Button::new(if is_auto_mount {
                            "Disable"
                        } else {
                            "Enable"
                        }))
                        .clicked()
                    {
                        if is_auto_mount {
                            disable_auto_mount(app);
                        } else {
                            enable_auto_mount(app);
                        }
                    }
                });
            });
    });
}
