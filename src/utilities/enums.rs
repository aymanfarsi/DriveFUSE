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
    EnableAutoMount,
    DisableAutoMount,
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
