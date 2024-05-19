use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    MountUnmount,
    Manage,
    Settings,
}
impl Tab {
    pub fn values() -> [Tab; 3] {
        [Tab::MountUnmount, Tab::Manage, Tab::Settings]
    }
    pub fn to_str(&self) -> &str {
        match self {
            Tab::MountUnmount => "Mount - Unmount",
            Tab::Manage => "Manage Storages",
            Tab::Settings => "Settings",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Quit,
    Icon,
    Green,
    Red,
    ShowApp,
    HideApp,
    RcloneConfigUpdated,
    // MountStorage(String),
    // UnmountStorage(String),
    MountAll,
    UnmountAll,
    // EnableAutoMount,
    // DisableAutoMount,
    // EnableAutoStart,
    // DisableAutoStart,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StorageType {
    GoogleDrive,
    OneDrive,
    Dropbox,
    GooglePhotos,
    Mega,
}

impl StorageType {
    pub fn name(&self) -> &str {
        match self {
            StorageType::GoogleDrive => "Google Drive",
            StorageType::OneDrive => "OneDrive",
            StorageType::Dropbox => "Dropbox",
            StorageType::GooglePhotos => "Google Photos",
            StorageType::Mega => "Mega",
        }
    }

    pub fn values() -> [StorageType; 5] {
        [
            StorageType::GoogleDrive,
            StorageType::OneDrive,
            StorageType::Dropbox,
            StorageType::GooglePhotos,
            StorageType::Mega,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AppTheme {
    #[serde(rename = "light")]
    Light,

    #[serde(rename = "dark")]
    Dark,

    #[serde(rename = "frappe")]
    FRAPPE,

    #[serde(rename = "latte")]
    LATTE,

    #[serde(rename = "macchiato")]
    MACCHIATO,

    #[serde(rename = "mocha")]
    MOCHA,
}

impl AppTheme {
    pub fn name(&self) -> &str {
        match self {
            AppTheme::Light => "Light",
            AppTheme::Dark => "Dark",
            AppTheme::FRAPPE => "Frappe",
            AppTheme::LATTE => "Latte",
            AppTheme::MACCHIATO => "Macchiato",
            AppTheme::MOCHA => "Mocha",
        }
    }

    pub fn values() -> [AppTheme; 6] {
        [
            AppTheme::Light,
            AppTheme::Dark,
            AppTheme::FRAPPE,
            AppTheme::LATTE,
            AppTheme::MACCHIATO,
            AppTheme::MOCHA,
        ]
    }

    pub fn set_theme(&self, ctx: &egui::Context) {
        match self {
            AppTheme::Light => {
                ctx.set_visuals(egui::Visuals::light());
            }
            AppTheme::Dark => {
                ctx.set_visuals(egui::Visuals::dark());
            }
            AppTheme::FRAPPE => {
                catppuccin_egui::set_theme(ctx, catppuccin_egui::FRAPPE);
            }
            AppTheme::LATTE => {
                catppuccin_egui::set_theme(ctx, catppuccin_egui::LATTE);
            }
            AppTheme::MACCHIATO => {
                catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO);
            }
            AppTheme::MOCHA => {
                catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA);
            }
        }
    }
}
