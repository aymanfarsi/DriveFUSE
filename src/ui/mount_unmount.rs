use egui::{CentralPanel, Context, RichText};
use rand::{seq::SliceRandom, thread_rng};

use crate::{utilities::utils::available_drives, RcloneApp};

pub fn render_mount_unmount(ctx: &Context, app: &mut RcloneApp) {
    CentralPanel::default().show(ctx, |ui| {
        ui.heading("RcloneApp");
        let count_storages = app.rclone.storages.len();
        ui.label(format!("Storages: {}", count_storages));
        egui::ScrollArea::new([false, true])
            .auto_shrink([false, true])
            .show(ui, |ui| {
                for storage in &app.rclone.storages {
                    ui.separator();
                    ui.label(format!("Name: {}", storage.name));
                    ui.label(format!("Type: {}", {
                        match storage.drive_type.as_str() {
                            "drive" => "Google Drive",
                            "onedrive" => "OneDrive",
                            _ => "Unknown",
                        }
                    }));
                    // ui.label(format!("Scope: {}", storage.scope));
                    // ui.label(format!(
                    //     "Token expiry: {:?}",
                    //     storage
                    //         .token
                    //         .expiry
                    //         .format("%A, %-d %B %Y at %H:%M:%S")
                    //         .to_string()
                    // ));
                    let is_mounted = app.mounted_storages.is_mounted(storage.name.clone());
                    if is_mounted {
                        ui.label(RichText::new("Mounted".to_string()).color(egui::Color32::GREEN));
                    } else {
                        ui.label(RichText::new("Unmounted".to_string()).color(egui::Color32::RED));
                    }
                    if ui
                        .button({
                            if is_mounted {
                                "Unmount"
                            } else {
                                "Mount"
                            }
                        })
                        .clicked()
                    {
                        if is_mounted {
                            app.mounted_storages.unmount(storage.name.clone());
                        } else {
                            let mut available_drives = available_drives();
                            available_drives.shuffle(&mut thread_rng());
                            let driver_letter = *available_drives.first().unwrap();
                            app.mounted_storages
                                .mount(driver_letter.to_string(), storage.name.clone());
                        }
                    }
                }
            });
    });
}
