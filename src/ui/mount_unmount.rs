use egui::{CentralPanel, Color32, Context, Grid, RichText, ScrollArea};

#[cfg(target_os = "windows")]
use {crate::utilities::utils::available_drives, egui::ComboBox};

use crate::{utilities::enums::AppTheme, RcloneApp};

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
                                "dropbox" => "Dropbox",
                                "googlephotos" => "Google Photos",
                                "mega" => "Mega",
                                _ => "Unknown",
                            };

                            ui.label(storage.name.to_string());
                            ui.label(drive_type);
                            ui.label(status_text);
                            let letter = app
                                .app_config
                                .get_drive_letter(&storage.name.clone())
                                .unwrap_or("N/A".to_string());
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

                            let mount_buttton = ui.button(action_text);
                            if mount_buttton.clicked() {
                                #[cfg(target_os = "windows")]
                                {
                                    if is_mounted {
                                        app.mounted_storages.unmount(storage.name.clone());
                                    } else {
                                        let is_drive_letter_mounted =
                                            app.mounted_storages.is_drive_letter_mounted(
                                                letter.clone().chars().next().unwrap(),
                                            );
                                        if letter != "N/A" && !is_drive_letter_mounted {
                                            app.mounted_storages.mount(
                                                letter.clone(),
                                                storage.name.clone(),
                                                false,
                                                &mut app.app_config,
                                            );
                                        } else if letter == "N/A" {
                                            let possible_drives = available_drives();
                                            let first_drive = possible_drives.first().unwrap();
                                            app.app_config.set_drives_letters(
                                                storage.name.clone(),
                                                *first_drive,
                                            );
                                            app.mounted_storages.mount(
                                                first_drive.to_string(),
                                                storage.name.clone(),
                                                false,
                                                &mut app.app_config,
                                            );
                                        } else {
                                            app.mounted_storages.mount(
                                                letter.clone(),
                                                storage.name.clone(),
                                                false,
                                                &mut app.app_config,
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
                                            false,
                                        );
                                    }
                                }
                            }
                            if mount_buttton.secondary_clicked() {
                                #[cfg(target_os = "windows")]
                                {
                                    if is_mounted {
                                        app.mounted_storages.unmount(storage.name.clone());
                                    } else {
                                        let is_already_mounted =
                                            app.mounted_storages.is_drive_letter_mounted(
                                                letter.clone().chars().next().unwrap(),
                                            );
                                        if letter != "N/A" && !is_already_mounted {
                                            app.mounted_storages.mount(
                                                letter,
                                                storage.name.clone(),
                                                true,
                                                &mut app.app_config,
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
                                            true,
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
