use std::{collections::HashMap, process::Command};

#[cfg(target_os = "windows")]
use {
    crate::utilities::utils::available_drives, directories::UserDirs,
    std::os::windows::process::CommandExt, winapi::um::winbase,
};

#[cfg(not(target_os = "windows"))]
use {std::fs::DirBuilder, std::path::Path};

use super::{app_config::AppConfig, rclone::Storage};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MountingStorage {
    drives: HashMap<String, u32>,
    mounted: HashMap<String, char>,
}

impl Default for MountingStorage {
    fn default() -> Self {
        let drives = HashMap::new();
        let mounted = HashMap::new();

        Self { drives, mounted }
    }
}

impl MountingStorage {
    pub fn total_mounted(&self) -> u32 {
        self.drives.len().try_into().unwrap()
    }

    pub fn is_drive_letter_mounted(&self, drive: char) -> bool {
        #[cfg(target_os = "linux")]
        let is_avail = false;
        #[cfg(target_os = "windows")]
        let is_avail = available_drives().contains(&drive);
        self.mounted.values().any(|&v| v == drive) || is_avail
    }

    pub fn is_mounted(&self, name: String) -> bool {
        #[cfg(target_os = "windows")]
        return self.mounted.contains_key(&name);

        #[cfg(target_os = "linux")]
        {
            let path = format!("/home/{}/drive_af/{}", whoami::username(), name.clone());
            let path = Path::new(&path);
            //if let Ok(entries) = fs::read_dir(path) {
            //    for entry in entries.flatten() {
            //        if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            //            return true;
            //        }
            //    }
            //}
            //false
            path.read_dir()
                .map(|mut i| i.next().is_some())
                .unwrap_or(false)
        }

        #[cfg(target_os = "macos")]
        {
            let path = format!("/Users/{}/drive_af/{}", whoami::username(), name.clone());
            let path = Path::new(&path);
            path.read_dir()
                .map(|mut i| i.next().is_some())
                .unwrap_or(false)
        }
    }

    pub fn get_mounted(&self, name: String) -> Option<String> {
        self.mounted.get(&name).map(|c| c.to_string())
    }

    pub fn mount_all(
        &mut self,
        drives: Vec<Storage>,
        drives_letters: HashMap<String, char>,
        network_mode: bool,
    ) -> bool {
        #[cfg(target_os = "windows")]
        {
            let mut success = true;
            // let mut available_drives = available_drives();
            for drive in drives {
                // let next_drive = available_drives.first().unwrap().to_string();
                let letter = drives_letters.get(&drive.name).unwrap().to_string();
                let id =
                    Self::mount_windows(drive.name.clone(), letter.clone(), false, network_mode);
                match id {
                    Some(id) => {
                        // available_drives.remove(0);
                        tracing::info!("Mounted {} to {}", drive.name, letter);
                        self.drives.insert(drive.name.clone(), id);
                        self.mounted
                            .insert(drive.name, letter.chars().next().unwrap());
                    }
                    None => {
                        tracing::error!("Failed to mount {} to {}", drive.name, letter);
                        success = false;
                    }
                }
            }
            success
        }

        #[cfg(not(target_os = "windows"))]
        {
            let mut success = true;
            let username = whoami::username();
            let root = if cfg!(target_os = "macos") {
                "/Users"
            } else {
                "/home"
            };

            for drive in drives {
                if !Path::new(&format!(
                    "{}/{}/drive_af/{}",
                    root,
                    username.clone(),
                    drive.name
                ))
                .exists()
                {
                    DirBuilder::new()
                        .recursive(true)
                        .create(format!("{}/{}/drive_af/{}", root, username, drive.name))
                        .unwrap();
                }
                let mut cmd = Command::new("rclone");
                let process = cmd
                    .arg("mount")
                    .arg(format!("{}:", drive.name))
                    .arg(format!(
                        "{}/{}/drive_af/{}",
                        root,
                        username.clone(),
                        drive.name
                    ))
                    .arg("--vfs-cache-mode")
                    .arg("full")
                    .arg("--dir-cache-time")
                    .arg("1000h")
                    .arg("--allow-other");

                let process = process.spawn();

                match process {
                    Ok(process) => {
                        tracing::info!(
                            "Mounted {} to {}/{}/drive_af/{}",
                            root,
                            username.clone(),
                            drive.name,
                            drive.name
                        );
                        self.drives.insert(drive.name.clone(), process.id());
                    }
                    Err(e) => {
                        tracing::error!(
                            "Error mounting {} at {}/{}/drive_af/{}: due to {}",
                            root,
                            username.clone(),
                            drive.name,
                            drive.name,
                            e
                        );
                        success = false;
                    }
                }
            }

            success
        }
    }

    pub fn unmount_all(&mut self) -> bool {
        #[cfg(target_os = "windows")]
        {
            let mut success = true;
            for (driver_letter, process_id) in self.drives.clone().iter() {
                let success_unmount = Self::unmount_windows(*process_id);
                if !success_unmount {
                    tracing::error!("Failed to unmount {}", driver_letter);
                    success = false;
                } else {
                    self.mounted.remove(driver_letter);
                    self.drives.remove(driver_letter);
                }
            }
            success
        }
        #[cfg(target_family = "unix")]
        {
            let mut success = true;
            for (name, process_id) in self.drives.iter() {
                let success_unmount = Self::unmount_unix(*process_id, name.to_string());
                if !success_unmount {
                    tracing::error!("Error unmounting {}", name);
                    success = false;
                }
            }
            success
        }
    }

    pub fn mount(
        &mut self,
        driver_letter: String,
        name: String,
        _show_terminal: bool,
        app_config: &mut AppConfig,
    ) {
        #[cfg(target_os = "windows")]
        {
            let id = Self::mount_windows(
                name.clone(),
                driver_letter.clone(),
                _show_terminal,
                app_config.enable_network_mode,
            );
            match id {
                Some(id) => {
                    tracing::info!("Mounted {} to {}", name, driver_letter);
                    self.drives.insert(name.clone(), id);
                    self.mounted
                        .insert(name.clone(), driver_letter.chars().next().unwrap());
                    app_config.set_drives_letters(name, driver_letter.chars().next().unwrap());
                }
                None => {
                    tracing::error!("Failed to mount {} to {}", name, driver_letter);
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            let username = whoami::username();
            let id = Self::mount_unix(name.clone());
            match id {
                Some(id) => {
                    tracing::info!("Mounted {} to /home/{}/drive_af/{}", username, name, name);
                    self.drives.insert(name.clone(), id);
                }
                None => {
                    tracing::error!(
                        "Failed to mount {} to /home/{}/drive_af/{}",
                        username,
                        name,
                        name
                    );
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            let username = whoami::username();
            let id = Self::mount_unix(name.clone());
            match id {
                Some(id) => {
                    tracing::info!("Mounted {} to /Users/{}/drive_af/{}", username, name, name);
                    self.drives.insert(name.clone(), id);
                }
                None => {
                    tracing::error!(
                        "Failed to mount {} to /Users/{}/drive_af/{}",
                        username,
                        name,
                        name
                    );
                }
            }
        }
    }

    pub fn unmount(&mut self, driver_letter: String) {
        #[cfg(target_os = "windows")]
        {
            let process_id = *self.drives.get(&driver_letter).unwrap();
            let success = Self::unmount_windows(process_id);
            if success {
                self.drives.remove(&driver_letter);
                self.mounted.remove(&driver_letter);
            } else {
                tracing::error!("Failed to unmount {}", driver_letter);
            }
        }
        #[cfg(target_os = "linux")]
        {
            let process_id = *self.drives.get(&driver_letter).unwrap();
            let success = Self::unmount_unix(process_id, driver_letter.clone());
            if success {
                #[cfg(target_os = "linux")]
                {
                    let _name = self
                        .drives
                        .iter()
                        .find(|(_, &v)| v == process_id)
                        .unwrap()
                        .0
                        .clone();
                    // unmount_delete_directory(name);
                    self.drives.remove(&driver_letter);
                }
            } else {
                tracing::error!("Failed to unmount {}", driver_letter);
            }
        }

        #[cfg(target_os = "macos")]
        {
            let process_id = *self.drives.get(&driver_letter).unwrap();
            let success = Self::unmount_unix(process_id, driver_letter.clone());
            if success {
                #[cfg(target_os = "macos")]
                {
                    let _name = self
                        .drives
                        .iter()
                        .find(|(_, &v)| v == process_id)
                        .unwrap()
                        .0
                        .clone();
                    // unmount_delete_directory(name);
                    self.drives.remove(&driver_letter);
                }
            } else {
                tracing::error!("Failed to unmount {}", driver_letter);
            }
        }
    }

    #[cfg(target_os = "windows")]
    fn mount_windows(
        name: String,
        driver_letter: String,
        show_terminal: bool,
        network_mode: bool,
    ) -> Option<u32> {
        let doc_app = UserDirs::new()
            .unwrap()
            .document_dir()
            .unwrap()
            .join("drive_af")
            .to_str()
            .unwrap()
            .to_owned();
        let mut cmd = Command::new("rclone");
        let process = cmd
            .arg("mount")
            .arg(format!("{}:", name))
            .arg(format!("{}:", driver_letter))
            .arg("--vfs-cache-mode")
            .arg("full")
            .arg("--volname")
            .arg(name.clone())
            .arg("--log-level")
            .arg("ERROR")
            .arg("--log-file")
            .arg(format!("{}/rclone-{}.log", doc_app, name));

        #[cfg(target_os = "windows")]
        if network_mode {
            process.arg("--network-mode");
        }

        if show_terminal {
            process.creation_flags(winbase::CREATE_NEW_CONSOLE);
        } else {
            process.creation_flags(winbase::CREATE_NO_WINDOW);
        }
        let process = process.spawn();

        match process {
            Ok(process) => Some(process.id()),
            Err(e) => {
                tracing::error!("Error mounting {} at {}: due to {}", name, driver_letter, e);
                None
            }
        }
    }

    #[cfg(target_os = "windows")]
    fn unmount_windows(id: u32) -> bool {
        let mut cmd = Command::new("taskkill");
        let process = cmd.arg("/F").arg("/PID").arg(&id.to_string());

        process.creation_flags(winbase::CREATE_NO_WINDOW);

        let mut process = process.spawn().expect("failed to execute process");

        let success = process.wait().expect("failed to wait on child");

        success.success()
    }

    #[cfg(target_os = "linux")]
    fn mount_unix(name: String) -> Option<u32> {
        let username = whoami::username();
        if !Path::new(&format!("/home/{}/drive_af/{}", username.clone(), name)).exists() {
            DirBuilder::new()
                .recursive(true)
                .create(format!("/home/{}/drive_af/{}", username, name))
                .unwrap();
        }
        let mut cmd = Command::new("rclone");
        let process = cmd
            .arg("mount")
            .arg(format!("{}:", name))
            .arg(format!("/home/{}/drive_af/{}", username, name))
            .arg("--vfs-cache-mode")
            .arg("full")
            //.arg("--dir-cache-time")
            //.arg("1000h")
            .arg("--allow-other");

        let process = process.spawn();

        match process {
            Ok(process) => Some(process.id()),
            Err(e) => {
                tracing::error!(
                    "Error mounting {} at /home/{}/drive_af/{}: due to {}",
                    username,
                    name,
                    name,
                    e
                );
                None
            }
        }
    }

    #[cfg(target_os = "macos")]
    fn mount_unix(name: String) -> Option<u32> {
        let username = whoami::username();
        if !Path::new(&format!("/Users/{}/drive_af/{}", username.clone(), name)).exists() {
            DirBuilder::new()
                .recursive(true)
                .create(format!("/Users/{}/drive_af/{}", username, name))
                .unwrap();
        }
        let mut cmd = Command::new("rclone");
        let process = cmd
            .arg("mount")
            .arg(format!("{}:", name))
            .arg(format!("/Users/{}/drive_af/{}", username, name))
            .arg("--vfs-cache-mode")
            .arg("full")
            //.arg("--dir-cache-time")
            //.arg("1000h")
            .arg("--allow-other");

        let process = process.spawn();

        match process {
            Ok(process) => Some(process.id()),
            Err(e) => {
                tracing::error!(
                    "Error mounting {} at /Users/{}/drive_af/{}: due to {}",
                    username,
                    name,
                    name,
                    e
                );
                None
            }
        }
    }

    #[cfg(target_family = "unix")]
    fn unmount_unix(_id: u32, name: String) -> bool {
        // let mut cmd = Command::new("kill");
        // let process = cmd.arg("-9").arg(&id.to_string());
        let root = if cfg!(target_os = "macos") {
            "/Users"
        } else {
            "/home"
        };

        let program = if cfg!(target_os = "linux") {
            "fusermount"
        } else {
            "umount"
        };

        let mut cmd: Command = Command::new(program);

        #[cfg(target_os = "linux")]
        cmd.arg("-u");

        cmd.arg(&format!(
            "{}/{}/drive_af/{}/",
            root,
            whoami::username(),
            name
        ));

        #[cfg(target_os = "linux")]
        cmd.arg("-z");

        let process = cmd.status();

        match process {
            Ok(code) => code.code() == Some(0),
            Err(err) => {
                tracing::error!("Error unmounting {} due to {}", name, err);
                false
            }
        }
    }
}
