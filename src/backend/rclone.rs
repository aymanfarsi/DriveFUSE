use chrono::{DateTime, FixedOffset};
use serde_json::Value;

#[cfg(target_os = "windows")]
use {std::os::windows::process::CommandExt, winapi::um::winbase};

use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    process::Command,
};

use crate::utilities::{
    enums::StorageType,
    utils::{
        add_dropbox_storage, add_google_drive_storage, add_google_photos_storage, add_mega_storage,
        add_onedrive_storage, app_config_path, rclone_config_path,
    },
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
            .truncate(false)
            .read(true)
            .open(rclone_config_path)
            .unwrap();
        let buffered = BufReader::new(file);

        let mut lines: Vec<String> = Vec::new();

        for line in buffered.lines() {
            lines.push(line.unwrap());
        }

        println!("Read app config");

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
            .open(rclone_config_path)
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
                line.get(1..line.len() - 1)
                    .unwrap()
                    .clone_into(&mut drive_name);
            } else {
                match line.get(..2) {
                    Some("ty") => line
                        .split('=')
                        .last()
                        .unwrap()
                        .replace(' ', "")
                        .clone_into(&mut drive_type),
                    Some("sc") => line
                        .split('=')
                        .last()
                        .unwrap()
                        .replace(' ', "")
                        .clone_into(&mut drive_scope),
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

        //for item in storages.clone() {
        //    let res = get_info(item.name.clone());
        //    match res {
        //        Ok(output) => {
        //            println!("{} has output of\n{}\n", item.name, output);
        //        }
        //        Err(err) => {
        //            println!("Error: {}", err);
        //        }
        //    }
        //}

        println!("Parsed rclone config file");

        storages
    }

    pub fn add_storage(&mut self, name: String, storage_type: StorageType) {
        match storage_type {
            StorageType::GoogleDrive => add_google_drive_storage(name),
            StorageType::OneDrive => add_onedrive_storage(name),
            StorageType::Dropbox => add_dropbox_storage(name),
            StorageType::GooglePhotos => add_google_photos_storage(name),
            StorageType::Mega => add_mega_storage(name),
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
        let mut cmd = Command::new("rclone");
        let output = cmd.arg("config").arg("delete").arg(name.clone());

        #[cfg(target_os = "windows")]
        output.creation_flags(winbase::CREATE_NO_WINDOW);

        let output = output.output().expect("failed to execute process");
        match String::from_utf8(output.stdout) {
            Ok(_) => self.storages.retain(|storage| storage.name != name),
            Err(_) => eprintln!("Error removing storage"),
        }
    }

    pub fn create_backup(&self) {
        let lines = Self::read_config();
        // let datetime = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
        // Self::write_config(lines, Some(format!("rclone_{}.conf", datetime)));
        tokio::spawn(async move {
            let res = rfd::AsyncFileDialog::new()
                .add_filter("rclone config", &["conf"])
                .set_directory(app_config_path().unwrap())
                .save_file()
                .await;

            if let Some(path) = res {
                Self::write_config(lines, Some(path.path().to_str().unwrap().to_owned()));
            }
        });
    }

    pub fn restore_backup(&self) {
        tokio::spawn(async move {
            let res = rfd::AsyncFileDialog::new()
                .add_filter("rclone config", &["conf"])
                .set_directory(app_config_path().unwrap())
                .pick_file()
                .await;

            if let Some(path) = res {
                let file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(false)
                    .open(rclone_config_path().unwrap().join("rclone.conf"))
                    .unwrap();
                let mut buffered = BufReader::new(file);

                let backup_file = OpenOptions::new().read(true).open(path.path()).unwrap();
                let backup_buffered = BufReader::new(backup_file);

                for line in backup_buffered.lines() {
                    buffered
                        .get_mut()
                        .write_all(line.unwrap().as_bytes())
                        .expect("couldnt write to file");
                    buffered
                        .get_mut()
                        .write_all("\n".as_bytes())
                        .expect("couldnt write to file");
                }
            }
        });
    }
}
