use bevy::prelude::*;

#[derive(Resource)]
pub struct Settings {
    first_run: bool,
    bgm_enabled: bool,
    se_enabled: bool,
    fullscreen_enabled: bool,
    bgm_volume: u8,
    se_volume: u8,
    sensitivity: u8,
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
                _ => println!("Invalid field"),
            }
        }
    }
    pub fn get_value(&self, field: &str) -> u8 {
        match field {
            "bgm" => self.bgm_volume,
            "se" => self.se_volume,
            "sensitivity" => self.sensitivity,
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
            _ => 0,
        }
    }
    fn is_value_vaild(value: i8) -> bool {
        (0..=100).contains(&value)
    }
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Settings {
            first_run: true,
            bgm_enabled: true,
            bgm_volume: 50,
            se_enabled: true,
            se_volume: 50,
            fullscreen_enabled: false,
            sensitivity: 50,
        });
    }
}
