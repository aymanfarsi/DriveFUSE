use std::process::Command;

use eframe::egui;
use egui::ViewportCommand;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tray_item::{IconSource, TrayItem};

#[cfg(target_os = "windows")]
use crate::utilities::tray_menu::init_tray_menu;

#[cfg(target_os = "linux")]
use tray_icon::menu::MenuEvent;

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

#[derive(Debug)]
pub struct DriveFUSE {
    pub app_config: AppConfig,
    pub rclone: Rclone,
    pub mounted_storages: MountingStorage,

    pub current_tab: Tab,

    pub new_storage_name: String,

    is_first_run: bool,
    is_close_requested: bool,

    pub tx_egui: UnboundedSender<Message>,
    rx_egui: UnboundedReceiver<Message>,

    pub platform: String,
}

impl DriveFUSE {
    pub fn new() -> Self {
        let (tx_egui, rx_egui) = mpsc::unbounded_channel();

        let app_config = AppConfig::init();
        let rclone = Rclone::init();

        let platform = if cfg!(target_os = "linux") {
            let cmd = "loginctl show-session $(awk '/tty/ {print $1}' <(loginctl)) -p Type | awk -F= '{print $2}'";
            let output = Command::new("bash")
                .arg("-c")
                .arg(cmd)
                .output()
                .expect("Unable to get platform");
            let platform = String::from_utf8(output.stdout).unwrap();

            let mut s = platform.trim().to_string();

            format!("{}{s}", s.remove(0).to_uppercase())
        } else if cfg!(target_os = "macos") {
            "macOS".to_string()
        } else {
            "Windows".to_string()
        };

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

            platform,
        }
    }
}

impl eframe::App for DriveFUSE {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                    let mut tray =
                        TrayItem::new("DriveFUSE Tray", icon).expect("Error creating tray item");
                    let mut rx_tray = init_tray_menu(&mut tray);
                    loop {
                        match rx_tray.recv().await {
                            Some(Message::Quit) => {
                                tx_egui_clone_tray
                                    .send(Message::ShowApp)
                                    .expect("Error sending ShowApp message to egui");
                                ctx_clone_tray.request_repaint();
                                tx_egui_clone_tray
                                    .send(Message::Quit)
                                    .expect("Error sending Quit message to egui");
                                ctx_clone_tray.request_repaint();
                                break;
                            }
                            Some(Message::Icon) => {
                                tray.set_icon(IconSource::Resource("app-icon"))
                                    .expect("Error setting tray icon");
                                tx_egui_clone_tray
                                    .send(Message::Icon)
                                    .expect("Error sending Icon message to egui");
                                ctx_clone_tray.request_repaint();
                            }
                            Some(Message::ShowApp) => {
                                tx_egui_clone_tray
                                    .send(Message::ShowApp)
                                    .expect("Error sending ShowApp message to egui");
                                ctx_clone_tray.request_repaint();
                            }
                            Some(Message::HideApp) => {
                                tx_egui_clone_tray
                                    .send(Message::HideApp)
                                    .expect("Error sending HideApp message to egui");
                                ctx_clone_tray.request_repaint();
                            }
                            Some(Message::RcloneConfigUpdated) => {
                                tracing::info!("Rclone config updated");
                            }
                            Some(Message::MountAll) => {
                                tx_egui_clone_tray
                                    .send(Message::MountAll)
                                    .expect("Error sending MountAll message to egui");
                                ctx_clone_tray.request_repaint();
                            }
                            Some(Message::UnmountAll) => {
                                tx_egui_clone_tray
                                    .send(Message::UnmountAll)
                                    .expect("Error sending UnmountAll message to egui");
                                ctx_clone_tray.request_repaint();
                            }
                            None => {
                                tracing::error!(
                                    "Error receiving message from tray menu. Channel closed"
                                );
                            }
                            _ => {
                                tracing::error!("Unhandeled message from tray menu");
                            }
                        }
                    }
                });
            }
            #[cfg(target_os = "linux")]
            {
                let tx_egui_clone_tray = self.tx_egui.clone();
                let ctx_clone_tray = ctx.clone();
                tokio::spawn(async move {
                    loop {
                        if let Ok(event) = MenuEvent::receiver().recv() {
                            let MenuEvent { id, .. } = event;
                            let id = id.0.as_str();
                            match id {
                                "quit" => {
                                    tx_egui_clone_tray
                                        .send(Message::Quit)
                                        .expect("Error sending Quit message to egui");
                                    ctx_clone_tray.request_repaint();
                                    break;
                                }
                                "show_app" => {
                                    tx_egui_clone_tray
                                        .send(Message::ShowApp)
                                        .expect("Error sending ShowApp message to egui");
                                    ctx_clone_tray.request_repaint();
                                }
                                "hide_app" => {
                                    tx_egui_clone_tray
                                        .send(Message::HideApp)
                                        .expect("Error sending HideApp message to egui");
                                    ctx_clone_tray.request_repaint();
                                }
                                "mount_all" => {
                                    tx_egui_clone_tray
                                        .send(Message::MountAll)
                                        .expect("Error sending MountAll message to egui");
                                    ctx_clone_tray.request_repaint();
                                }
                                "unmount_all" => {
                                    tx_egui_clone_tray
                                        .send(Message::UnmountAll)
                                        .expect("Error sending UnmountAll message to egui");
                                    ctx_clone_tray.request_repaint();
                                }
                                _ => panic!("Unknown menu item"),
                            }
                        }
                    }
                });
            }

            // * Spawn rclone config watcher thread
            let tx_egui_clone_config = self.tx_egui.clone();
            let ctx_clone_config = ctx.clone();
            tokio::spawn(async move {
                let (tx_temp, mut rx_temp) = mpsc::unbounded_channel();
                let mut watcher: RecommendedWatcher = RecommendedWatcher::new(
                    move |res| {
                        tx_temp
                            .send(res)
                            .expect("Error sending message to watcher thread");
                    },
                    Config::default(),
                )
                .expect("Error creating watcher");
                watcher
                    .watch(
                        rclone_config_path()
                            .expect("Error getting rclone config path")
                            .as_path(),
                        RecursiveMode::Recursive,
                    )
                    .expect("Error watching rclone config file");
                loop {
                    match rx_temp.recv().await {
                        Some(res) => match res {
                            Ok(event) => {
                                if event.kind.is_modify() {
                                    tx_egui_clone_config
                                        .send(Message::RcloneConfigUpdated)
                                        .expect(
                                            "Error sending RcloneConfigUpdated message to egui",
                                        );
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

        // * Handle messages
        if let Ok(message) = self.rx_egui.try_recv() {
            match message {
                Message::Quit => {
                    tracing::info!("Quit message received");

                    ctx.send_viewport_cmd(ViewportCommand::Visible(true));
                    ctx.request_repaint();

                    self.is_close_requested = true;
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
                Message::Icon => {
                    tracing::info!("Icon message received");

                    ctx.send_viewport_cmd(ViewportCommand::Title("DriveFUSE".to_string()));
                }
                Message::ShowApp => {
                    tracing::info!("ShowApp message received");

                    ctx.send_viewport_cmd(ViewportCommand::Visible(true));
                    ctx.send_viewport_cmd(ViewportCommand::Focus);
                }
                Message::HideApp => {
                    tracing::info!("HideApp message received");

                    ctx.send_viewport_cmd(ViewportCommand::Visible(false));
                }
                Message::RcloneConfigUpdated => {
                    tracing::info!("RcloneConfigUpdated message received");

                    self.rclone = Rclone::init();
                }
                Message::MountAll => {
                    tracing::info!("MountAll message received");

                    self.mounted_storages.mount_all(
                        self.rclone.storages.clone(),
                        self.app_config.drives_letters.clone(),
                        self.app_config.enable_network_mode,
                    );

                    ctx.request_repaint();
                }
                Message::UnmountAll => {
                    tracing::info!("UnmountAll message received");

                    self.mounted_storages.unmount_all();

                    ctx.request_repaint();
                }
                Message::MountedSuccess => {
                    tracing::info!("MountedSuccess message received");

                    ctx.request_repaint();
                }
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

        // * Check if close requested
        if ctx.input(|i| i.viewport().close_requested()) {
            match self.is_close_requested {
                true => match self.mounted_storages.unmount_all() {
                    true => {
                        tracing::info!("Unmounted all drives and quitting");
                        std::process::exit(0);
                    }
                    false => {
                        self.is_close_requested = false;
                        ctx.send_viewport_cmd(ViewportCommand::CancelClose);
                    }
                },
                false => {
                    self.tx_egui
                        .send(Message::HideApp)
                        .expect("Error sending HideApp message to egui");
                    self.is_close_requested = false;
                    ctx.send_viewport_cmd(ViewportCommand::CancelClose);
                }
            }
        }
    }
}
