use egui::{menu, Align, Context, Layout, RichText, TextStyle, TopBottomPanel};

use crate::{
    utilities::enums::{Message, Tab},
    RcloneApp,
};

pub fn render_top_panel(ctx: &Context, app: &mut RcloneApp) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.add_space(3.);
        ui.horizontal_wrapped(|ui| {
            ui.visuals_mut().button_frame = false;
            menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                    let text = format!(
                        "{}/{}",
                        app.mounted_storages.total_mounted(),
                        app.rclone.storages.len()
                    );
                    let label = ui.label(RichText::new(text).size(14.).strong());
                    label.context_menu(|ui| {
                        if ui.button("Mount all").clicked() {
                            app.mounted_storages.unmount_all();
                            app.mounted_storages.mount_all(app.rclone.storages.clone());
                            ui.close_menu();
                        }
                        if ui.button("Unmount all").clicked() {
                            app.mounted_storages.unmount_all();
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("Quit").clicked() {
                            app.tx_egui.send(Message::Quit).unwrap();
                            ui.close_menu();
                        }
                    });
                    ui.add_space(3.);
                    for tab in Tab::values() {
                        let btn = ui.selectable_label(
                            app.current_tab == tab,
                            RichText::new(tab.to_str())
                                .size(14.)
                                .text_style(TextStyle::Button),
                        );
                        if btn.clicked() {
                            app.current_tab = tab;
                        }
                    }
                    ui.add_space(3.);
                });
            });
        });
        ui.add_space(3.);
    });
}
