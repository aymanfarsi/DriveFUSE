use std::{collections::HashMap, os::windows::process::CommandExt, process::Command};

use winapi::um::winbase;

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
        self.drives.contains_key(&name)
    }

    pub fn get_mounted(&self, name: String) -> Option<String> {
        self.mounted.get(&name).map(|c| c.to_string())
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
        let process = Command::new("rclone")
            .arg("mount")
            .arg(format!("{}:", name))
            .arg(format!("{}:", driver_letter))
            .arg("--vfs-cache-mode")
            .arg("full")
            .arg("--volname")
            .arg(name.clone())
            .creation_flags(winbase::CREATE_NO_WINDOW)
            .spawn();

        match process {
            Ok(process) => Some(process.id()),
            Err(e) => {
                eprintln!("Error mounting {} at {}: due to {}", name, driver_letter, e);
                None
            }
        }
    }

    fn unmount_windows(id: u32) -> bool {
        let mut process = Command::new("taskkill")
            .arg("/F")
            .arg("/PID")
            .arg(&id.to_string())
            .creation_flags(winbase::CREATE_NO_WINDOW)
            .spawn()
            .expect("failed to execute process");

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
