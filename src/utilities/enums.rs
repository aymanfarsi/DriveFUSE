#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Message {
    Quit,
    Green,
    Red,
    ShowApp,
    HideApp,
    RcloneConfigUpdated,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StorageType{
    GoogleDrive,
    OneDrive,
}