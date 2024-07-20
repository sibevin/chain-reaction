use crate::app;
use bevy::prelude::*;
use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Resource, Serialize, Deserialize)]
pub struct Settings {
    first_run: bool,
    bgm_enabled: bool,
    se_enabled: bool,
    fullscreen_enabled: bool,
    bgm_volume: u8,
    se_volume: u8,
    sensitivity: u8,
    sensitivity_modified: u8,
    last_player: String,
}

impl Settings {
    pub fn is_enabled(&self, field: &str) -> bool {
        match field {
            "first" => self.first_run,
            "bgm" => self.bgm_enabled,
            "se" => self.se_enabled,
            "fullscreen" => self.fullscreen_enabled,
            _ => false,
        }
    }
    pub fn toggle(&mut self, field: &str) {
        match field {
            "first" => self.first_run = !self.first_run,
            "bgm" => self.bgm_enabled = !self.bgm_enabled,
            "se" => self.se_enabled = !self.se_enabled,
            "fullscreen" => self.fullscreen_enabled = !self.fullscreen_enabled,
            _ => println!("Invalid field"),
        }
    }
    pub fn set_value(&mut self, field: &str, value: i8) {
        if Settings::is_value_vaild(value) {
            match field {
                "bgm" => self.bgm_volume = value as u8,
                "se" => self.se_volume = value as u8,
                "sensitivity" => self.sensitivity = value as u8,
                "sensitivity_modified" => self.sensitivity_modified = value as u8,
                _ => println!("Invalid field"),
            }
        }
    }
    pub fn get_value(&self, field: &str) -> u8 {
        match field {
            "bgm" => self.bgm_volume,
            "se" => self.se_volume,
            "sensitivity" => self.sensitivity,
            "sensitivity_modified" => self.sensitivity_modified,
            _ => 0,
        }
    }
    pub fn current_value(&self, field: &str) -> u8 {
        match field {
            "bgm" => {
                if self.bgm_enabled {
                    self.bgm_volume
                } else {
                    0
                }
            }
            "se" => {
                if self.se_enabled {
                    self.se_volume
                } else {
                    0
                }
            }
            "sensitivity" => self.sensitivity,
            "sensitivity_modified" => self.sensitivity_modified,
            _ => 0,
        }
    }
    pub fn fetch_last_player(&self) -> &str {
        &self.last_player
    }
    pub fn update_last_player(&mut self, name: &str) {
        self.last_player = String::from(name);
    }
    fn is_value_vaild(value: i8) -> bool {
        (0..=100).contains(&value)
    }
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        let config_dir = dirs::config_dir()
            .map(|native_config_dir| native_config_dir.join(app::APP_CODE))
            .unwrap_or(Path::new("local").join("configuration"));
        app.insert_resource(
            Persistent::<Settings>::builder()
                .name("settings")
                .format(StorageFormat::Json)
                .path(config_dir.join("settings.json"))
                .default(Settings {
                    first_run: true,
                    bgm_enabled: true,
                    bgm_volume: 50,
                    se_enabled: true,
                    se_volume: 50,
                    fullscreen_enabled: false,
                    sensitivity: 50,
                    sensitivity_modified: 10,
                    last_player: String::from(""),
                })
                .revertible(true)
                .revert_to_default_on_deserialization_errors(true)
                .build()
                .expect("failed to initialize variables"),
        );
    }
}
