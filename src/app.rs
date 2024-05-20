use std::sync::mpsc::{Receiver, SyncSender};

use eframe::egui;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
#[cfg(target_os = "windows")]
use tray_item::{IconSource, TrayItem};

#[cfg(target_os = "windows")]
use crate::utilities::tray_menu::init_tray_menu;
use crate::{
    backend::{
        app_config::AppConfig,
        mounting::MountingStorage,
        rclone::{Rclone, Storage},
    },
    ui::{
        manage::render_manage, mount_unmount::render_mount_unmount, settings::render_settings,
        top_panel::render_top_panel,
    },
    utilities::{
        enums::{Message, Tab},
        utils::rclone_config_path,
    },
};

pub struct RcloneApp {
    pub app_config: AppConfig,
    pub rclone: Rclone,
    pub mounted_storages: MountingStorage,

    pub current_tab: Tab,

    // pub selected_storage: Option<String>,
    pub new_storage_name: String,
    // pub new_storage_drive_letter: String,
    // pub edit_storage_name: String,
    is_first_run: bool,
    is_close_requested: bool,

    pub tx_egui: SyncSender<Message>,
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

        let app_config = AppConfig::init();
        let rclone = Rclone::init();

        Self {
            app_config,
            rclone,
            mounted_storages: MountingStorage::default(),

            current_tab: Tab::MountUnmount,

            // selected_storage: None,
            new_storage_name: String::new(),
            // new_storage_drive_letter: String::from("N/A"),
            // edit_storage_name: String::new(),
            is_first_run: true,
            is_close_requested: false,

            tx_egui,
            rx_egui,
        }
    }
}

impl eframe::App for RcloneApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // * First run setup
        if self.is_first_run {
            self.is_first_run = false;

            // * Set theme
            self.app_config.current_theme.set_theme(ctx);

            // * Spawn tray menu thread on first run
            #[cfg(target_os = "windows")]
            {
                let tx_egui_clone_tray = self.tx_egui.clone();
                let ctx_clone_tray = ctx.clone();
                tokio::spawn(async move {
                    let icon = IconSource::Resource("app-icon");
                    let mut tray = TrayItem::new("DriveAF Tray", icon).unwrap();
                    let rx_tray = init_tray_menu(&mut tray);
                    loop {
                        match rx_tray.recv() {
                            Ok(Message::Quit) => {
                                tx_egui_clone_tray.send(Message::ShowApp).unwrap();
                                ctx_clone_tray.request_repaint();
                                tx_egui_clone_tray.send(Message::Quit).unwrap();
                                ctx_clone_tray.request_repaint();
                                break;
                            }
                            Ok(Message::Icon) => {
                                tray.set_icon(IconSource::Resource("app-icon")).unwrap();
                                tx_egui_clone_tray.send(Message::Icon).unwrap();
                                ctx_clone_tray.request_repaint();
                            }
                            Ok(Message::Red) => {
                                tray.set_icon(IconSource::Resource("red-icon")).unwrap();
                                tx_egui_clone_tray.send(Message::Red).unwrap();
                                ctx_clone_tray.request_repaint();
                            }
                            Ok(Message::Green) => {
                                tx_egui_clone_tray.send(Message::Green).unwrap();
                                tray.set_icon(IconSource::Resource("green-icon")).unwrap();
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
                                tracing::error!("Error receiving message from tray menu");
                            }
                            Ok(Message::RcloneConfigUpdated) => {
                                tracing::info!("Rclone config updated");
                            }
                            Ok(Message::MountAll) => {
                                tx_egui_clone_tray.send(Message::MountAll).unwrap();
                                ctx_clone_tray.request_repaint();
                            }
                            Ok(Message::UnmountAll) => {
                                tx_egui_clone_tray.send(Message::UnmountAll).unwrap();
                                ctx_clone_tray.request_repaint();
                            } // Ok(Message::EnableAutoMount) => {
                              //     tx_egui_clone_tray.send(Message::EnableAutoMount).unwrap();
                              //     ctx_clone_tray.request_repaint();
                              // }
                              // Ok(Message::DisableAutoMount) => {
                              //     tx_egui_clone_tray.send(Message::DisableAutoMount).unwrap();
                              //     ctx_clone_tray.request_repaint();
                              // }
                              // Ok(Message::EnableAutoStart) => {
                              //     tx_egui_clone_tray.send(Message::EnableAutoStart).unwrap();
                              //     ctx_clone_tray.request_repaint();
                              // }
                              // Ok(Message::DisableAutoStart) => {
                              //     tx_egui_clone_tray.send(Message::DisableAutoStart).unwrap();
                              //     ctx_clone_tray.request_repaint();
                              // }
                              // Ok(Message::MountStorage(name)) => {
                              //     tx_egui_clone_tray
                              //         .send(Message::MountStorage(name))
                              //         .unwrap();
                              //     ctx_clone_tray.request_repaint();
                              // }
                              // Ok(Message::UnmountStorage(name)) => {
                              //     tx_egui_clone_tray
                              //         .send(Message::UnmountStorage(name))
                              //         .unwrap();
                              //     ctx_clone_tray.request_repaint();
                              // }
                        }
                    }
                });
            }

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
                                tracing::error!("Error watching rclone config file");
                            }
                        },
                        None => {
                            tracing::error!("Error receiving message from rclone config watcher. Channel closed");
                        }
                    }
                }
            });

            // * Auto mount drives on startup
            if self.app_config.is_auto_mount {
                let mut drives: Vec<Storage> = vec![];
                for storage in self.rclone.storages.clone() {
                    if let Some(drive) = self.app_config.get_drive_auto_mount(&storage.name) {
                        if drive {
                            drives.push(storage.clone())
                        }
                    }
                }
                self.mounted_storages.mount_all(
                    drives,
                    self.app_config.drives_letters.clone(),
                    self.app_config.enable_network_mode,
                );
            }
        }

        // * Handle messages from tray menu
        if let Ok(message) = self.rx_egui.try_recv() {
            match message {
                Message::Quit => {
                    // ! unmount all drives
                    self.is_close_requested = true;
                    frame.close();
                }
                Message::Icon => {
                    frame.set_window_title("DriveAF");
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
                Message::MountAll => {
                    self.mounted_storages.mount_all(
                        self.rclone.storages.clone(),
                        self.app_config.drives_letters.clone(),
                        self.app_config.enable_network_mode,
                    );
                }
                Message::UnmountAll => {
                    self.mounted_storages.unmount_all();
                } // Message::EnableAutoMount => {
                  //     self.app_config.set_is_auto_mount(true);
                  // }
                  // Message::DisableAutoMount => {
                  //     self.app_config.set_is_auto_mount(false);
                  // }
                  // Message::EnableAutoStart => {
                  //     enable_auto_start_app();
                  // }
                  // Message::DisableAutoStart => {
                  //     disable_auto_start_app();
                  // }
                  // Message::MountStorage(name) => {
                  //     let mut letter = self
                  //         .app_config
                  //         .get_drive_letter(&name)
                  //         .unwrap_or("N/A".to_string());
                  //     if letter == "N/A" {
                  //         let possible = available_drives();
                  //         letter = possible.first().unwrap().to_string();
                  //     }
                  //     self.mounted_storages.mount(
                  //         letter.clone(),
                  //         name,
                  //         false,
                  //         &mut self.app_config,
                  //     );
                  // }
                  // Message::UnmountStorage(name) => {
                  //     self.mounted_storages.unmount(name);
                  // }
            }
        }

        // * Top panel
        render_top_panel(ctx, self);

        // * Tab content
        match self.current_tab {
            Tab::MountUnmount => render_mount_unmount(ctx, self),
            Tab::Manage => render_manage(ctx, self),
            Tab::Settings => render_settings(ctx, self),
        };
    }

    fn on_close_event(&mut self) -> bool {
        #[cfg(target_os = "linux")]
        match self.mounted_storages.unmount_all() {
            true => true,
            false => {
                tracing::error!("Failed to unmount all drives");
                false
            }
        }
        #[cfg(target_os = "windows")]
        match self.is_close_requested {
            true => match self.mounted_storages.unmount_all() {
                true => true,
                false => {
                    tracing::error!("Failed to unmount all drives");
                    self.is_close_requested = false;
                    false
                }
            },
            false => {
                self.tx_egui.send(Message::HideApp).unwrap();
                self.is_close_requested = false;
                false
            }
        }
        #[cfg(target_os = "macos")]
        match self.mounted_storages.unmount_all() {
            true => true,
            false => {
                tracing::error!("Failed to unmount all drives");
                false
            }
        }
    }
}
