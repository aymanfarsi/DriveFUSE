use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
};

use serde::{Deserialize, Serialize};

use crate::utilities::utils::app_config_path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    pub is_first_run: bool,
    pub is_auto_mount: bool,
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

    pub fn save(&self) {
        let config_path = app_config_path().unwrap();

        if !config_path.exists() {
            fs::create_dir_all(&config_path).unwrap()
        }

        let mut file = File::create(config_path.join("config.json")).unwrap();
        let json = serde_json::to_string_pretty(&self).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}
