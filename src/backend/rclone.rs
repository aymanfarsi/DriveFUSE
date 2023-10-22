use chrono::{DateTime, FixedOffset};
use serde_json::Value;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    process::Command,
};

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

    fn parse_storages(&self) -> Vec<Storage> {
        let rclone_config_path = rclone_config_path().unwrap().join("rclone.conf");
        let file = OpenOptions::new()
            .write(false)
            .create(false)
            .read(true)
            .open(&rclone_config_path)
            .unwrap();
        let buffered = BufReader::new(file);

        let mut drive_name = String::new();
        let mut drive_type = String::new();
        let mut drive_scope = String::new();
        let mut drive_token: Option<TokenStruct> = None;

        let mut storages: Vec<Storage> = Vec::new();

        for line in buffered.lines() {
            let line = line.unwrap();
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

    pub fn edit_storage_name(&mut self, name: String, storage: Storage) {
        // change name in rclone config
        let output = Command::new("rclone")
            .arg("config")
            .arg("update")
            .arg(name.clone())
            .arg("name")
            .arg(storage.name.clone())
            .output()
            .expect("failed to execute process");
        match String::from_utf8(output.stdout) {
            Ok(_) => {
                let index = self
                    .storages
                    .iter()
                    .position(|storage| storage.name == name)
                    .unwrap();
                self.storages[index] = storage;
            }
            Err(_) => eprintln!("Error renaming storage"),
        }
    }

    pub fn remove_storage(&mut self, name: String) {
        let output = Command::new("rclone")
            .arg("config")
            .arg("delete")
            .arg(name.clone())
            .output()
            .expect("failed to execute process");
        match String::from_utf8(output.stdout) {
            Ok(_) => self.storages.retain(|storage| storage.name != name),
            Err(_) => eprintln!("Error removing storage"),
        }
    }
}
