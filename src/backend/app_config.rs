use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
};

use serde::{Deserialize, Serialize};

use crate::utilities::utils::app_config_path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    pub is_first_run: bool,
}

impl AppConfig {
    pub fn init() -> Self {
        let config_path = app_config_path().unwrap();

        if !config_path.exists() {
            fs::create_dir_all(&config_path).unwrap();
            let mut file = File::create(&config_path.join("config.json")).unwrap();

            let json = serde_json::to_string_pretty(&AppConfig { is_first_run: true }).unwrap();

            file.write_all(json.as_bytes()).unwrap();
        }

        let file = File::open(&config_path.join("config.json")).unwrap();
        let reader = BufReader::new(file);

        let mut lines: Vec<String> = Vec::new();

        for line in reader.lines() {
            lines.push(line.unwrap());
        }

        let json = lines.join("\n");

        serde_json::from_str(&json).unwrap()
    }
}
