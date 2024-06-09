use egui::{menu, vec2, Align, Button, Layout, RichText, TextStyle, TopBottomPanel};

use crate::{utilities::enums::Tab, RcloneApp};

pub fn render_top_panel(ctx: &egui::Context, app: &mut RcloneApp) {
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
                    ui.label(RichText::new(text).size(14.).strong());

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

                    let available_space = ui.available_width();
                    ui.add_space(available_space - 50.);

                    let hide_btn = ui.add_sized(
                        vec2(50., 20.),
                        Button::new(if app.app_config.hide_storage_label {
                            "Show"
                        } else {
                            "Hide"
                        }),
                    );
                    if hide_btn.clicked() {
                        app.app_config.hide_storage_label = !app.app_config.hide_storage_label;
                    }
                });
            });
        });
        ui.add_space(3.);
    });
}
