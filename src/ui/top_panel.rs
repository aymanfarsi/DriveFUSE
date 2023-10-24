use egui::{menu, Align, Context, Layout, RichText, TextStyle, TopBottomPanel};

use crate::{utilities::enums::Tab, RcloneApp};

pub fn render_top_panel(ctx: &Context, app: &mut RcloneApp) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.add_space(3.);
        ui.horizontal_wrapped(|ui| {
            ui.visuals_mut().button_frame = false;
            menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                    ui.label(
                        RichText::new(app.mounted_storages.total_mounted().to_string())
                            .size(14.)
                            .strong(),
                    );
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
