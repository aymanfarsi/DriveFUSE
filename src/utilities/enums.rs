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
pub enum StorageType{
    GoogleDrive,
    OneDrive,
}