use std::sync::mpsc::{Receiver, SyncSender};

use eframe::egui;

use crate::utils::enums::Message;

pub struct RcloneApp {
    name: String,
    age: u32,
    rx_main: Receiver<Message>,
}

impl RcloneApp {
    pub fn new() -> (Self, SyncSender<Message>) {
        let (tx_main, rx_main) = std::sync::mpsc::sync_channel(1);

        (
            Self {
                name: "evilDAVE".to_owned(),
                age: 24,
                rx_main,
            },
            tx_main,
        )
    }
}

impl eframe::App for RcloneApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Handle incoming messages from the main thread.
        while let Ok(message) = self.rx_main.try_recv() {
            match message {
                Message::Quit => {
                    frame.close();
                }
                Message::Red => {
                    frame.set_window_title("RcloneApp - Red");
                }
                Message::Green => {
                    frame.set_window_title("RcloneApp - Green");
                }
                _ => {}
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    frame.close();
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
