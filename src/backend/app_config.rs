use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
};

use serde::{Deserialize, Serialize};

use crate::utilities::{enums::AppTheme, utils::app_config_path};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub is_first_run: bool,
    pub is_auto_mount: bool,

    pub current_theme: AppTheme,
    pub hide_storage_label: bool,

    pub drives_letters: HashMap<String, char>,
    pub drives_auto_mount: HashMap<String, bool>,
}

impl AppConfig {
    pub fn init() -> Self {
        let config_path = app_config_path().unwrap();

        if !config_path.exists() {
            fs::create_dir_all(&config_path).unwrap()
        }

        let file = &config_path.join("config.json");
        if !file.exists() {
            let mut file = File::create(file).unwrap();
            let json = serde_json::to_string_pretty(&AppConfig {
                is_first_run: true,
                is_auto_mount: false,
                current_theme: AppTheme::Dark,
                hide_storage_label: false,
                drives_letters: HashMap::new(),
                drives_auto_mount: HashMap::new(),
            })
            .unwrap();
            file.write_all(json.as_bytes()).unwrap();
        }

        let file = File::open(file).unwrap();
        let reader = BufReader::new(file);

        let mut lines: Vec<String> = Vec::new();

        for line in reader.lines() {
            lines.push(line.unwrap());
        }

        let json = lines.join("\n");

        serde_json::from_str(&json).unwrap()
    }

    fn save(&self) {
        let config_path = app_config_path().unwrap();

        if !config_path.exists() {
            fs::create_dir_all(&config_path).unwrap()
        }

        let mut file = File::create(config_path.join("config.json")).unwrap();
        let json = serde_json::to_string_pretty(&self).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn set_is_first_run(&mut self, is_first_run: bool) {
        self.is_first_run = is_first_run;
        self.save();
    }

    pub fn set_is_auto_mount(&mut self, is_auto_mount: bool) {
        self.is_auto_mount = is_auto_mount;
        self.save();
    }

    pub fn set_current_theme(&mut self, current_theme: AppTheme) {
        self.current_theme = current_theme;
        self.save();
    }

    pub fn set_hide_storage_label(&mut self, hide_storage_label: bool) {
        self.hide_storage_label = hide_storage_label;
        self.save();
    }

    pub fn set_drives_letters(&mut self, key: String, value: char) {
        self.drives_letters.insert(key, value);
        self.save();
    }

    pub fn get_drive_letter(&self, key: &str) -> Option<String> {
        self.drives_letters.get(key).map(|c| c.to_string())
    }

    pub fn set_drives_auto_mount(&mut self, key: String, value: bool) {
        self.drives_auto_mount.insert(key, value);
        self.save();
    }

    pub fn get_drive_auto_mount(&self, key: &str) -> Option<bool> {
        self.drives_auto_mount.get(key).copied()
    }
}
