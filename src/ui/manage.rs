use egui::{CentralPanel, ComboBox, Context, RichText, ScrollArea};

use crate::{backend::rclone::Storage, utilities::enums::StorageType, RcloneApp};

pub fn render_manage(ctx: &Context, app: &mut RcloneApp) {
    CentralPanel::default().show(ctx, |ui| {
        // * Label
        ui.heading(RichText::new("Manage your rclone storages").size(21.0));

        ui.add_space(8.0);

        ScrollArea::new([false, true])
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                // * Add storage
                ui.horizontal(|ui| {
                    ui.label("Storage name: ");
                    ui.text_edit_singleline(&mut app.new_storage_name);
                });

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    if ui.button("Google Drive").clicked() {
                        app.rclone
                            .add_storage(app.new_storage_name.clone(), StorageType::GoogleDrive);
                        app.new_storage_name = String::new();
                    }
                    ui.add_space(8.0);
                    if ui.button("OneDrive").clicked() {
                        app.rclone
                            .add_storage(app.new_storage_name.clone(), StorageType::OneDrive);
                        app.new_storage_name = String::new();
                    }
                });

                ui.add_space(8.0);

                // * Select storage
                ComboBox::from_label("")
                    .selected_text({
                        match app.selected_storage.clone() {
                            Some(storage) => storage,
                            None => "Select storage".to_string(),
                        }
                    })
                    .show_ui(ui, |ui| {
                        for storage in &app.rclone.storages {
                            let resp = ui.selectable_value(
                                &mut app.selected_storage,
                                Some(storage.name.clone()),
                                storage.name.clone(),
                            );
                            if resp.clicked() {
                                app.edit_storage_name = storage.name.clone();
                                ui.close_menu();
                            }
                        }
                    });

                ui.add_space(8.0);

                // * Edit storage
                match app.selected_storage.clone() {
                    Some(name) => {
                        let storage = app
                            .rclone
                            .storages
                            .iter()
                            .find(|s| s.name == name)
                            .unwrap()
                            .clone();

                        ui.horizontal(|ui| {
                            ui.label("Name:");
                            ui.text_edit_singleline(&mut app.edit_storage_name);
                        });

                        ui.add_space(8.0);

                        if ui.button("Edit name (double ckick)").double_clicked() {
                            let old_name = storage.name.clone();
                            let new_name = app.edit_storage_name.clone();

                            let index = app
                                .rclone
                                .storages
                                .iter()
                                .position(|temp| temp.name == old_name)
                                .unwrap();
                            app.rclone.storages[index] = Storage {
                                name: new_name.clone(),
                                drive_type: app.rclone.storages[index].drive_type.clone(),
                                scope: app.rclone.storages[index].scope.clone(),
                                token: app.rclone.storages[index].token.clone(),
                            };

                            app.rclone.edit_storage_name(old_name, new_name.clone());
                            app.edit_storage_name = String::new();
                            app.selected_storage = Some(new_name);
                        }

                        ui.add_space(8.0);

                        if ui.button("Delete storage (double ckick)").double_clicked() {
                            app.rclone.remove_storage(storage.name.clone());
                        }
                    }
                    None => {
                        ui.label("Please select a storage to edit its name");
                    }
                }
            });
    });
}
