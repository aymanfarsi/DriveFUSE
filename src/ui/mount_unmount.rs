use egui::{vec2, Button, CentralPanel, Color32, Context, Grid, RichText, Rounding, ScrollArea};

#[cfg(target_os = "windows")]
use {crate::utilities::utils::available_drives, egui::ComboBox};

#[cfg(target_family = "unix")]
use crate::utilities::utils::open_drive_location;

use crate::{utilities::enums::AppTheme, DriveFUSE};

pub fn render_mount_unmount(ctx: &Context, app: &mut DriveFUSE) {
    CentralPanel::default().show(ctx, |ui| {
        //        ui.heading(
        //            RichText::new(format!(
        //                "You ve got {} storages ^_^",
        //                app.rclone.storages.len()
        //            ))
        //            .size(21.0),
        //        );
        //        ui.add_space(8.0);
        ScrollArea::new([false, true])
            .auto_shrink([false, true])
            .drag_to_scroll(false)
            .show(ui, |ui| {
                Grid::new("storage_grid")
                    .striped(app.app_config.current_theme == AppTheme::Dark)
                    .num_columns(4)
                    .spacing([8.0, 8.0])
                    .min_col_width(80.0)
                    .show(ui, |ui| {
                        ui.label("Name");

                        ui.label("Type");

                        ui.label("Status");

                        #[cfg(target_os = "windows")]
                        ui.label("Drive Letter");
                        #[cfg(target_family = "unix")]
                        ui.label("Location");

                        ui.label("Action | Auto Mount");

                        ui.end_row();

                        for storage in &app.rclone.storages {
                            let is_mounted = app.mounted_storages.is_mounted(storage.name.clone());

                            #[cfg(target_os = "windows")]
                            let possible_drives = available_drives();

                            let status_text = if is_mounted {
                                RichText::new("Mounted").color(Color32::GREEN)
                            } else {
                                RichText::new("Unmounted").color(Color32::RED)
                            };

                            let action_text = if is_mounted { "Unmount" } else { "Mount" };

                            let drive_type = match storage.drive_type.as_str() {
                                "drive" => "Google Drive",
                                "onedrive" => "OneDrive",
                                "dropbox" => "Dropbox",
                                "googlephotos" => "Google Photos",
                                "mega" => "Mega",
                                "nextcloud" => "NextCloud",
                                _ => "Unknown",
                            };

                            ui.label(if app.app_config.hide_storage_label {
                                "*".repeat(storage.name.len())
                            } else {
                                storage.name.clone()
                            });
                            ui.label(drive_type);
                            ui.label(status_text);
                            #[cfg(target_os = "windows")]
                            let letter = app
                                .app_config
                                .get_drive_letter(&storage.name.clone())
                                .expect("N/A");
                            #[cfg(target_os = "windows")]
                            if is_mounted {
                                ui.label(letter.clone());
                            } else {
                                ComboBox::from_id_source(format!("drive_letter_{}", storage.name))
                                    .selected_text(letter.clone())
                                    .width(70.)
                                    .show_ui(ui, |ui| {
                                        for drive in &possible_drives {
                                            let selected_value = drive.to_string();
                                            let text = drive.to_string();

                                            let response =
                                                ui.selectable_label(selected_value == letter, text);

                                            if response.clicked() {
                                                app.app_config.set_drives_letters(
                                                    storage.name.clone(),
                                                    *drive,
                                                );
                                                ui.close_menu();
                                            }
                                        }
                                    });
                            }

                            #[cfg(target_family = "unix")]
                            ui.horizontal(|ui| {
                                ui.add_enabled_ui(is_mounted, |ui| {
                                    let open_button = ui.add_sized(
                                        vec2(60.0, 20.0),
                                        Button::new(RichText::new("Open"))
                                            .frame(true)
                                            .rounding(Rounding::same(5.)),
                                    );

                                    if open_button.clicked() {
                                        open_drive_location(storage.name.clone());
                                    }
                                });
                            });

                            ui.horizontal(|ui| {
                                let mount_buttton = ui.add_sized(
                                    vec2(60.0, 20.0),
                                    Button::new(RichText::new(action_text).color(if is_mounted {
                                        Color32::WHITE
                                    } else {
                                        Color32::BLACK
                                    }))
                                    .frame(true)
                                    .fill(if is_mounted {
                                        Color32::RED
                                    } else {
                                        Color32::LIGHT_GREEN
                                    })
                                    .rounding(Rounding::same(5.)),
                                );
                                if mount_buttton.clicked() {
                                    #[cfg(target_os = "windows")]
                                    {
                                        if is_mounted {
                                            app.mounted_storages.unmount(storage.name.clone());
                                        } else {
                                            let is_drive_letter_mounted =
                                                app.mounted_storages.is_drive_letter_mounted(
                                                    letter.clone().chars().next().expect(
                                                        "Couldn't get drive letter from chars",
                                                    ),
                                                );
                                            if letter != "N/A" && !is_drive_letter_mounted {
                                                app.mounted_storages.mount(
                                                    letter.clone(),
                                                    storage.name.clone(),
                                                    false,
                                                    &mut app.app_config,
                                                    app.tx_egui.clone(),
                                                );
                                            } else if letter == "N/A" {
                                                let possible_drives = available_drives();
                                                let first_drive = possible_drives
                                                    .first()
                                                    .expect("No available drives");
                                                app.app_config.set_drives_letters(
                                                    storage.name.clone(),
                                                    *first_drive,
                                                );
                                                app.mounted_storages.mount(
                                                    first_drive.to_string(),
                                                    storage.name.clone(),
                                                    false,
                                                    &mut app.app_config,
                                                    app.tx_egui.clone(),
                                                );
                                            } else {
                                                app.mounted_storages.mount(
                                                    letter.clone(),
                                                    storage.name.clone(),
                                                    false,
                                                    &mut app.app_config,
                                                    app.tx_egui.clone(),
                                                );
                                            }
                                        }
                                    }
                                    #[cfg(target_family = "unix")]
                                    {
                                        if is_mounted {
                                            app.mounted_storages.unmount(storage.name.clone());
                                        } else {
                                            app.mounted_storages.mount(
                                                String::from("N/A"),
                                                storage.name.clone(),
                                                false,
                                                &mut app.app_config,
                                                app.tx_egui.clone(),
                                            );
                                        }
                                    }
                                }

                                let storage_auto_mount = app
                                    .app_config
                                    .get_drive_auto_mount(&storage.name.clone())
                                    .unwrap_or(false);
                                let auto_mount = ui.add(
                                    Button::new(
                                        RichText::new(if storage_auto_mount {
                                            "Yes"
                                        } else {
                                            "No"
                                        })
                                        .color(
                                            if storage_auto_mount {
                                                Color32::BLACK
                                            } else {
                                                Color32::WHITE
                                            },
                                        ),
                                    )
                                    .fill(if storage_auto_mount {
                                        Color32::LIGHT_GREEN
                                    } else {
                                        Color32::RED
                                    })
                                    .frame(!storage_auto_mount)
                                    .rounding(Rounding::same(5.))
                                    .min_size(vec2(50.0, 20.0)),
                                );
                                if auto_mount.clicked() {
                                    app.app_config.set_drives_auto_mount(
                                        storage.name.clone(),
                                        !storage_auto_mount,
                                    );
                                }
                            });

                            ui.end_row();
                        }
                    });
            });
    });
}
