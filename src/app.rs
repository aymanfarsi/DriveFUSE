use std::{
    sync::mpsc::{Receiver, SyncSender},
    thread,
};

use eframe::egui;
use tray_item::{IconSource, TrayItem};

use crate::utils::{enums::Message, tray_menu::init_tray_menu};

pub struct RcloneApp {
    name: String,
    age: u32,

    is_first_run: bool,

    tx_egui: SyncSender<Message>,
    rx_egui: Receiver<Message>,
}

impl Default for RcloneApp {
    fn default() -> Self {
        Self::new()
    }
}

impl RcloneApp {
    pub fn new() -> Self {
        let (tx_egui, rx_egui) = std::sync::mpsc::sync_channel(1);

        Self {
            name: "evilDAVE".to_owned(),
            age: 24,

            is_first_run: true,

            tx_egui,
            rx_egui,
        }
    }
}

impl eframe::App for RcloneApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.is_first_run {
            self.is_first_run = false;

            let tx_egui_clone = self.tx_egui.clone();
            let ctx_clone = ctx.clone();

            thread::spawn(move || {
                let mut tray =
                    TrayItem::new("RcloneApp Tray", IconSource::Resource("green-icon-file"))
                        .unwrap();
                let rx_tray = init_tray_menu(&mut tray);
                loop {
                    match rx_tray.recv() {
                        Ok(Message::Quit) => {
                            println!("Quit");
                            tx_egui_clone.send(Message::Quit).unwrap();
                            ctx_clone.request_repaint();
                            break;
                        }
                        Ok(Message::Red) => {
                            println!("Red");
                            tray.set_icon(IconSource::Resource("red-icon-file"))
                                .unwrap();
                            tx_egui_clone.send(Message::Red).unwrap();
                            ctx_clone.request_repaint();
                        }
                        Ok(Message::Green) => {
                            println!("Green");
                            tx_egui_clone.send(Message::Green).unwrap();
                            tray.set_icon(IconSource::Resource("green-icon-file"))
                                .unwrap();
                            ctx_clone.request_repaint();
                        }
                        Ok(Message::ShowApp) => {
                            println!("Show");
                            tx_egui_clone.send(Message::ShowApp).unwrap();
                            ctx_clone.request_repaint();
                        }
                        Ok(Message::HideApp) => {
                            println!("Hide");
                            tx_egui_clone.send(Message::HideApp).unwrap();
                            ctx_clone.request_repaint();
                        }
                        Err(_) => {
                            eprintln!("Error receiving message from tray menu");
                        }
                    }
                }
            });
        }

        while let Ok(message) = self.rx_egui.try_recv() {
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
                Message::ShowApp => {
                    frame.set_visible(true);
                }
                Message::HideApp => {
                    frame.set_visible(false);
                }
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Show/Hide").clicked() {
                    frame.set_visible(false);
                }

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
