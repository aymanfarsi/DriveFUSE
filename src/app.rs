use std::sync::mpsc::{Receiver, SyncSender};

use eframe::egui;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tray_item::{IconSource, TrayItem};

use crate::{
    backend::rclone::Rclone,
    utilities::{enums::Message, tray_menu::init_tray_menu, utils::rclone_config_path},
};

pub struct RcloneApp {
    rclone: Rclone,

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

        let rclone = Rclone::init();

        Self {
            rclone,

            is_first_run: true,

            tx_egui,
            rx_egui,
        }
    }
}

impl eframe::App for RcloneApp {
    fn on_close_event(&mut self) -> bool {
        self.tx_egui.send(Message::HideApp).unwrap();
        false
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.is_first_run {
            self.is_first_run = false;

            // * Spawn tray menu thread on first run
            let tx_egui_clone_tray = self.tx_egui.clone();
            let ctx_clone_tray = ctx.clone();
            tokio::spawn(async move {
                let mut tray =
                    TrayItem::new("RcloneApp Tray", IconSource::Resource("green-icon-file"))
                        .unwrap();
                let rx_tray = init_tray_menu(&mut tray);
                loop {
                    match rx_tray.recv() {
                        Ok(Message::Quit) => {
                            tx_egui_clone_tray.send(Message::Quit).unwrap();
                            ctx_clone_tray.request_repaint();
                            break;
                        }
                        Ok(Message::Red) => {
                            tray.set_icon(IconSource::Resource("red-icon-file"))
                                .unwrap();
                            tx_egui_clone_tray.send(Message::Red).unwrap();
                            ctx_clone_tray.request_repaint();
                        }
                        Ok(Message::Green) => {
                            tx_egui_clone_tray.send(Message::Green).unwrap();
                            tray.set_icon(IconSource::Resource("green-icon-file"))
                                .unwrap();
                            ctx_clone_tray.request_repaint();
                        }
                        Ok(Message::ShowApp) => {
                            tx_egui_clone_tray.send(Message::ShowApp).unwrap();
                            ctx_clone_tray.request_repaint();
                        }
                        Ok(Message::HideApp) => {
                            tx_egui_clone_tray.send(Message::HideApp).unwrap();
                            ctx_clone_tray.request_repaint();
                        }
                        Err(_) => {
                            eprintln!("Error receiving message from tray menu");
                        }
                        Ok(Message::RcloneConfigUpdated) => {}
                    }
                }
            });

            // * Spawn rclone config watcher thread
            let tx_egui_clone_config = self.tx_egui.clone();
            let ctx_clone_config = ctx.clone();
            tokio::spawn(async move {
                let (tx_temp, mut rx_temp) = tokio::sync::mpsc::channel(1);
                let mut watcher: RecommendedWatcher = RecommendedWatcher::new(
                    move |res| {
                        tx_temp.blocking_send(res).unwrap();
                    },
                    Config::default(),
                )
                .unwrap();
                watcher
                    .watch(
                        rclone_config_path().unwrap().as_path(),
                        RecursiveMode::Recursive,
                    )
                    .unwrap();
                loop {
                    match rx_temp.recv().await {
                        Some(res) => match res {
                            Ok(event) => {
                                if event.kind.is_modify() {
                                    tx_egui_clone_config
                                        .send(Message::RcloneConfigUpdated)
                                        .unwrap();
                                    ctx_clone_config.request_repaint();
                                }
                            }
                            Err(_) => {
                                eprintln!("Error receiving message from rclone config watcher");
                            }
                        },
                        None => {
                            eprintln!("Channel closed");
                        }
                    }
                }
            });
        }

        // * Handle messages from tray menu
        if let Ok(message) = self.rx_egui.try_recv() {
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
                Message::RcloneConfigUpdated => {
                    self.rclone = Rclone::init();
                }
            }
        }

        // * Top panel
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Hide").clicked() {
                    self.tx_egui.send(Message::HideApp).unwrap();
                    ui.close_menu();
                }
            });
        });

        // * Central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("RcloneApp");
            let count_storages = self.rclone.storages.len();
            ui.label(format!("Storages: {}", count_storages));
            egui::ScrollArea::new([false, true])
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    for storage in &self.rclone.storages {
                        ui.separator();
                        ui.label(format!("Name: {}", storage.name));
                        ui.label(format!("Type: {}", storage.drive_type));
                        ui.label(format!("Scope: {}", storage.scope));
                        ui.label(format!(
                            "Token expiry: {:?}",
                            storage
                                .token
                                .expiry
                                .format("%A, %-d %B %Y at %H:%M:%S")
                                .to_string()
                        ));
                    }
                });
        });
    }
}
