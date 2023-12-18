use std::{collections::HashMap, process::Command};

use directories::UserDirs;

#[cfg(target_os = "windows")]
use {
    crate::utilities::utils::available_drives, std::os::windows::process::CommandExt,
    winapi::um::winbase,
};

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

    pub fn is_mounted(&self, name: String) -> bool {
        self.mounted.contains_key(&name)
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

        #[cfg(target_os = "linux")]
        {
            eprintln!("Linux is not supported yet!");
            false
        }

        #[cfg(target_os = "macos")]
        {
            eprintln!("MacOS is not supported yet!");
            false
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
                eprintln!("This platform {} is not supported yet!", platform);
                false
            }
        }
    }

    pub fn mount(&mut self, driver_letter: String, name: String) {
        let platform = std::env::consts::OS;
        match platform {
            "windows" => {
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
            _ => {
                eprintln!("This platform {} is not supported yet!", platform);
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
                eprintln!("This platform {} is not supported yet!", platform);
            }
        }
    }

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

    // MOUNTING
    // .arg("--allow-other")

    // UNMOUNTING
    // # Linux
    // fusermount -u /path/to/local/mount
    // # OS X
    // umount /path/to/local/mount
}
