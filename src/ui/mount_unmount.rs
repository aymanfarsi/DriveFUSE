use egui::{CentralPanel, Color32, Context, Grid, RichText, ScrollArea};

#[cfg(target_os = "windows")]
use {crate::utilities::utils::available_drives, egui::ComboBox};

use crate::RcloneApp;

pub fn render_mount_unmount(ctx: &Context, app: &mut RcloneApp) {
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
            .show(ui, |ui| {
                Grid::new("storage_grid")
                    .striped(true)
                    .num_columns(4)
                    .spacing([8.0, 8.0])
                    .min_col_width(80.0)
                    .show(ui, |ui| {
                        ui.label("Name");
                        ui.label("Type");
                        ui.label("Status");
                        #[cfg(target_os = "windows")]
                        ui.label("Drive Letter");
                        ui.label("Action");
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
                                _ => "Unknown",
                            };

                            ui.label(storage.name.to_string());
                            ui.label(drive_type);
                            ui.label(status_text);
                            #[cfg(target_os = "windows")]
                            if is_mounted {
                                ui.label(
                                    app.mounted_storages
                                        .get_mounted(storage.name.clone())
                                        .unwrap_or("N/A".to_owned()),
                                );
                            } else {
                                ComboBox::from_id_source(format!("drive_letter_{}", storage.name))
                                    .selected_text(app.new_storage_drive_letter.clone())
                                    .width(70.)
                                    .show_ui(ui, |ui| {
                                        for drive in &possible_drives {
                                            let current_value = &mut app.new_storage_drive_letter;
                                            let selected_value = drive.to_string();
                                            let text = drive.to_string();

                                            let response = ui.selectable_label(
                                                *current_value == selected_value,
                                                text,
                                            );

                                            if response.clicked() {
                                                *current_value = selected_value;
                                                ui.close_menu();
                                            }
                                        }
                                    });
                            }

                            if ui.button(action_text).clicked() {
                                #[cfg(target_os = "windows")]
                                {
                                    if is_mounted {
                                        app.mounted_storages.unmount(storage.name.clone());
                                    } else {
                                        // let drive_letter = app.new_storage_drive_letter.clone();
                                        // let is_mounted = app.mounted_storages.is_drive_letter_mounted(
                                        //     drive_letter.chars().next().unwrap(),
                                        // );
                                        // if is_mounted {
                                        //     let possible_drives = available_drives();
                                        //     app.new_storage_drive_letter =
                                        //         possible_drives.first().unwrap().to_string();
                                        // }
                                        let is_already_mounted =
                                            app.mounted_storages.is_drive_letter_mounted(
                                                app.new_storage_drive_letter
                                                    .chars()
                                                    .next()
                                                    .unwrap(),
                                            );
                                        if app.new_storage_drive_letter != "N/A"
                                            && !is_already_mounted
                                        {
                                            app.mounted_storages.mount(
                                                app.new_storage_drive_letter.clone(),
                                                storage.name.clone(),
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
                                            app.new_storage_drive_letter.clone(),
                                            storage.name.clone(),
                                        );
                                    }
                                }
                            }
                            ui.end_row();
                        }
                    });
            });
    });
}
