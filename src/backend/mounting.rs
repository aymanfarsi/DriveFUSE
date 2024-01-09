use std::{collections::HashMap, process::Command};

use directories::UserDirs;

#[cfg(target_os = "windows")]
use {
    crate::utilities::utils::available_drives, std::os::windows::process::CommandExt,
    winapi::um::winbase,
};

#[cfg(not(target_os = "windows"))]
use {std::fs, std::fs::DirBuilder, std::path::Path};

#[cfg(target_os = "linux")]
use crate::utilities::utils::unmount_delete_directory;

use super::rclone::Storage;

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
        self.mounted.values().any(|&v| v == drive)
    }

    pub fn is_mounted(&self, name: String) -> bool {
        #[cfg(target_os = "windows")]
        return self.mounted.contains_key(&name);

        #[cfg(not(target_os = "windows"))]
        {
            let path = format!("/home/{}/drive_af/{}", whoami::username(), name.clone());
            let path = Path::new(&path);
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                        return true;
                    }
                }
            }
            false
        }
    }

    pub fn get_mounted(&self, name: String) -> Option<String> {
        self.mounted.get(&name).map(|c| c.to_string())
    }

    pub fn mount_all(&mut self, drives: Vec<Storage>) -> bool {
        #[cfg(target_os = "windows")]
        {
            let mut success = true;
            let mut available_drives = available_drives();
            for drive in drives {
                let next_drive = available_drives.first().unwrap().to_string();
                let id = Self::mount_windows(drive.name.clone(), next_drive.clone());
                match id {
                    Some(id) => {
                        available_drives.remove(0);
                        println!("Mounted {} to {}", drive.name, next_drive);
                        self.drives.insert(drive.name.clone(), id);
                        self.mounted
                            .insert(drive.name, next_drive.chars().next().unwrap());
                    }
                    None => {
                        eprintln!("Failed to mount {} to {}", drive.name, next_drive);
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

            for drive in drives {
                if !Path::new(&format!(
                    "/home/{}/drive_af/{}",
                    username.clone(),
                    drive.name
                ))
                .exists()
                {
                    DirBuilder::new()
                        .recursive(true)
                        .create(format!("/home/{}/drive_af/{}", username, drive.name))
                        .unwrap();
                }
                let mut cmd = Command::new("rclone");
                let process = cmd
                    .arg("mount")
                    .arg(format!("{}:", drive.name))
                    .arg(format!(
                        "/home/{}/drive_af/{}",
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
                        println!(
                            "Mounted {} to /home/{}/drive_af/{}",
                            username.clone(),
                            drive.name,
                            drive.name
                        );
                        self.drives.insert(drive.name.clone(), process.id());
                    }
                    Err(e) => {
                        eprintln!(
                            "Error mounting {} at /home/{}/drive_af/{}: due to {}",
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

    pub fn unmount_all(&self) -> bool {
        let platform = std::env::consts::OS;
        match platform {
            "windows" => {
                let mut success = true;
                for (driver_letter, process_id) in self.drives.iter() {
                    let success_unmount = Self::unmount_windows(*process_id);
                    if !success_unmount {
                        eprintln!("Failed to unmount {}", driver_letter);
                        success = false;
                    }
                }
                success
            }
            _ => {
                let mut success = true;
                for (name, process_id) in self.drives.iter() {
                    let mut cmd = Command::new("kill");
                    let process = cmd.arg("-9").arg(&process_id.to_string());

                    let process = process.spawn();

                    match process {
                        Ok(_) => {
                            // #[cfg(target_os = "linux")]
                            // unmount_delete_directory(name.clone());
                        }
                        Err(e) => {
                            eprintln!("Error unmounting {} due to {}", name, e);
                            success = false;
                        }
                    }
                }
                success
            }
        }
    }

    pub fn mount(&mut self, driver_letter: String, name: String) {
        #[cfg(target_os = "windows")]
        {
            let id = Self::mount_windows(name.clone(), driver_letter.clone());
            match id {
                Some(id) => {
                    println!("Mounted {} to {}", name, driver_letter);
                    self.drives.insert(name.clone(), id);
                    self.mounted
                        .insert(name, driver_letter.chars().next().unwrap());
                }
                None => {
                    eprintln!("Failed to mount {} to {}", name, driver_letter);
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            let username = whoami::username();
            let id = Self::mount_unix(name.clone());
            match id {
                Some(id) => {
                    println!("Mounted {} to /home/{}/drive_af/{}", username, name, name);
                    self.drives.insert(name.clone(), id);
                }
                None => {
                    eprintln!(
                        "Failed to mount {} to /home/{}/drive_af/{}",
                        username, name, name
                    );
                }
            }
        }
    }

    pub fn unmount(&mut self, driver_letter: String) {
        let platform = std::env::consts::OS;
        match platform {
            "windows" => {
                let process_id = *self.drives.get(&driver_letter).unwrap();
                let success = Self::unmount_windows(process_id);
                if success {
                    self.drives.remove(&driver_letter);
                    self.mounted.remove(&driver_letter);
                } else {
                    eprintln!("Failed to unmount {}", driver_letter);
                }
            }
            _ => {
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
                    eprintln!("Failed to unmount {}", driver_letter);
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    fn mount_windows(name: String, driver_letter: String) -> Option<u32> {
        let doc_app = UserDirs::new()
            .unwrap()
            .document_dir()
            .unwrap()
            .join("rclone_app")
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
            // .arg("--dir-cache-time")
            // .arg("1000h")
            .arg("--log-level")
            .arg("NOTICE")
            .arg("--log-file")
            .arg(format!("{}/rclone-{}.log", doc_app, name));
        // .arg("--network-mode");
        // .arg("--vfs-cache-max-size")
        // .arg("100G")
        // .arg("--drive-chunk-size")
        // .arg("32M")
        // .arg("--buffer-size")
        // .arg("64M")

        #[cfg(target_os = "windows")]
        process.creation_flags(winbase::CREATE_NO_WINDOW);
        let process = process.spawn();

        match process {
            Ok(process) => Some(process.id()),
            Err(e) => {
                eprintln!("Error mounting {} at {}: due to {}", name, driver_letter, e);
                None
            }
        }
    }

    fn unmount_windows(id: u32) -> bool {
        let mut cmd = Command::new("taskkill");
        let process = cmd.arg("/F").arg("/PID").arg(&id.to_string());

        #[cfg(target_os = "windows")]
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
            .arg("--dir-cache-time")
            .arg("1000h")
            .arg("--allow-other");

        let process = process.spawn();

        match process {
            Ok(process) => Some(process.id()),
            Err(e) => {
                eprintln!(
                    "Error mounting {} at /home/{}/drive_af/{}: due to {}",
                    username, name, name, e
                );
                None
            }
        }
    }

    fn unmount_unix(_id: u32, name: String) -> bool {
        // let mut cmd = Command::new("kill");
        // let process = cmd.arg("-9").arg(&id.to_string());
        let mut cmd = Command::new("fusermount");
        let process = cmd.args([
            "-u",
            &format!("/home/{}/drive_af/{}/", whoami::username(), name),
        ]);
        let process = process.status();

        match process {
            Ok(code) => {
                println!("fusermount exitcode: {}", code);
                code.code() == Some(0)
            }
            Err(err) => {
                eprintln!("Error unmounting {} due to {}", name, err);
                false
            }
        }
    }

    // UNMOUNTING
    // # Linux
    // fusermount -u /path/to/local/mount
    // # OS X
    // umount /path/to/local/mount
}
