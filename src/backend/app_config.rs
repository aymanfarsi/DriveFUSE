use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
};

use serde::{Deserialize, Serialize};

use crate::utilities::utils::app_config_path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    pub is_first_run: bool,
    pub is_auto_mount: bool,
    pub drives_letters: HashMap<String, char>,
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
                drives_letters: HashMap::new(),
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

    pub fn set_drives_letters(&mut self, key: String, value: char) {
        self.drives_letters.insert(key, value);
        self.save();
    }

    pub fn get_drive_letter(&self, key: &str) -> Option<String> {
        self.drives_letters.get(key).map(|c| c.to_string())
    }
}
