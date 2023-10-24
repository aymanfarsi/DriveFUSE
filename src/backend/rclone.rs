use chrono::{DateTime, FixedOffset};
use serde_json::Value;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    os::windows::process::CommandExt,
    process::Command,
};
use winapi::um::winbase;

use crate::utilities::{
    enums::StorageType,
    utils::{add_google_drive_storage, add_onedrive_storage, rclone_config_path},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rclone {
    pub storages: Vec<Storage>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Storage {
    pub name: String,
    pub drive_type: String,
    pub scope: String,
    pub token: TokenStruct,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenStruct {
    access_token: String,
    pub token_type: String,
    refresh_token: String,
    pub expiry: DateTime<FixedOffset>,
}

impl Rclone {
    pub fn init() -> Self {
        let mut rclone = Self { storages: vec![] };

        rclone.storages = rclone.parse_storages();

        rclone
    }

    fn read_config() -> Vec<String> {
        let rclone_config_path = rclone_config_path().unwrap().join("rclone.conf");
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(&rclone_config_path)
            .unwrap();
        let buffered = BufReader::new(file);

        let mut lines: Vec<String> = Vec::new();

        for line in buffered.lines() {
            lines.push(line.unwrap());
        }

        lines
    }

    fn write_config(lines: Vec<String>, file_name: Option<String>) {
        let rclone_config_path = rclone_config_path().unwrap().join({
            match file_name.clone() {
                Some(val) => val,
                None => "rclone.conf".to_string(),
            }
        });
        let file = OpenOptions::new()
            .write(true)
            .create(file_name.is_some())
            .truncate(true)
            .open(&rclone_config_path)
            .unwrap();
        let mut buffered = BufReader::new(file);

        for line in lines {
            buffered
                .get_mut()
                .write_all(line.as_bytes())
                .expect("couldnt write to file");
            buffered
                .get_mut()
                .write_all("\n".as_bytes())
                .expect("couldnt write to file");
        }
    }

    fn parse_storages(&self) -> Vec<Storage> {
        let mut drive_name = String::new();
        let mut drive_type = String::new();
        let mut drive_scope = String::new();
        let mut drive_token: Option<TokenStruct> = None;

        let mut storages: Vec<Storage> = Vec::new();

        for line in Self::read_config() {
            if line.is_empty() {
                storages.push(Storage {
                    name: drive_name.clone(),
                    drive_type: drive_type.clone(),
                    scope: drive_scope.clone(),
                    token: match drive_token {
                        Some(ref val) => val.clone(),
                        None => panic!("didnt get token for driver"),
                    },
                });
            } else if line.starts_with('[') {
                drive_name = line.get(1..line.len() - 1).unwrap().to_owned();
            } else {
                match line.get(..2) {
                    Some("ty") => {
                        drive_type = line.split('=').last().unwrap().replace(' ', "").to_owned()
                    }
                    Some("sc") => {
                        drive_scope = line.split('=').last().unwrap().replace(' ', "").to_owned()
                    }
                    Some("to") => {
                        let input = line.split('=').last().unwrap();
                        let json: Value =
                            serde_json::from_str(input).expect("couldnt parse token json");
                        drive_token = Some(TokenStruct {
                            access_token: json["access_token"].to_string().replace('\"', ""),
                            token_type: json["token_type"].to_string().replace('\"', ""),
                            refresh_token: json["refresh_token"].to_string().replace('\"', ""),
                            expiry: DateTime::parse_from_rfc3339(
                                &json["expiry"].to_string().replace('\"', ""),
                            )
                            .unwrap(),
                        });
                    }
                    _ => continue,
                }
            }
        }

        storages
    }

    pub fn add_storage(&mut self, name: String, storage_type: StorageType) {
        match storage_type {
            StorageType::GoogleDrive => add_google_drive_storage(name),
            StorageType::OneDrive => add_onedrive_storage(name),
        }
    }

    pub fn edit_storage_name(&mut self, old_name: String, new_name: String) {
        let lines = Self::read_config();

        let mut new_lines: Vec<String> = Vec::new();

        let mut is_updated = false;

        for line in lines {
            if line.starts_with('[') && !is_updated {
                if line.get(1..line.len() - 1).unwrap() == old_name {
                    new_lines.push(format!("[{}]", new_name));
                    is_updated = true;
                } else {
                    new_lines.push(line);
                }
            } else {
                new_lines.push(line);
            }
        }

        match is_updated {
            true => Self::write_config(new_lines, None),
            false => eprintln!("Error updating storage name"),
        }
    }

    pub fn remove_storage(&mut self, name: String) {
        let output = Command::new("rclone")
            .arg("config")
            .arg("delete")
            .arg(name.clone())
            .creation_flags(winbase::CREATE_NO_WINDOW)
            .output()
            .expect("failed to execute process");
        match String::from_utf8(output.stdout) {
            Ok(_) => self.storages.retain(|storage| storage.name != name),
            Err(_) => eprintln!("Error removing storage"),
        }
    }

    pub fn create_backup(&self) {
        let lines = Self::read_config();
        let datetime = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
        Self::write_config(lines, Some(format!("rclone_{}.conf", datetime)));
    }
}