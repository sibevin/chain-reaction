use crate::reactor::status;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_persistent::prelude::*;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub mod achievement_xxx;
pub mod alpha_xxx;
pub mod c_chain_xxx;
pub mod h_chain_xxx;
pub mod max_c_xxx;
pub mod max_h;
pub mod not_moving_xxx_s;
pub mod score_xxx;
pub mod time_xxx_s;

pub const ACHIEVEMENTS: [&dyn AchievementDefBase; 9] = [
    &max_h::AchievementDef,
    &max_c_xxx::AchievementDef,
    &not_moving_xxx_s::AchievementDef,
    &alpha_xxx::AchievementDef,
    &score_xxx::AchievementDef,
    &time_xxx_s::AchievementDef,
    &h_chain_xxx::AchievementDef,
    &c_chain_xxx::AchievementDef,
    &achievement_xxx::AchievementDef,
];

const MAX_RUNNING_ACH_COUNT: usize = 2;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AchievementRecord {
    pub code: String,
    pub is_done: bool,
    pub done_at: String,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct AchievementStore {
    pub is_enabled: bool,
    pinned_codes: Vec<String>,
    record_map: HashMap<String, AchievementRecord>,
}

impl AchievementStore {
    pub fn toggle_pin(&mut self, code: &str) -> bool {
        let record = self.fetch_record(code);
        if record.is_done || self.pinned_codes.contains(&String::from(code)) {
            self.pinned_codes.retain(|pinned_code| *pinned_code != code);
            false
        } else {
            self.pinned_codes.insert(0, String::from(code));
            self.pinned_codes.truncate(MAX_RUNNING_ACH_COUNT);
            self.pinned_codes.shrink_to_fit();
            true
        }
    }
    pub fn mark_done(&mut self, code: &str) {
        let mut record = self.fetch_record(code);
        if !record.is_done {
            record.is_done = true;
            record.done_at = Local::now().format("%Y-%m-%d_%H:%M:%S%.9f").to_string();
            self.record_map.insert(record.code.clone(), record);
        }
    }
    pub fn is_done(&self, code: &str) -> bool {
        self.fetch_record(code).is_done
    }
    pub fn is_pinned(&self, code: &str) -> bool {
        let record = self.fetch_record(code);
        if record.is_done {
            false
        } else {
            self.pinned_codes.contains(&String::from(code))
        }
    }
    pub fn pinned_codes(&self) -> Vec<String> {
        self.pinned_codes.clone()
    }
    pub fn fetch_record(&self, code: &str) -> AchievementRecord {
        if let Some(record) = self.record_map.get(code) {
            record.clone()
        } else {
            let ach_def = fetch_ach_def(code);
            ach_def.build_empty_record()
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum AchievementUiStatus {
    Normal,
    Pinned,
    Done,
}

#[derive(Clone, PartialEq, Debug)]
pub enum AchievementProgressUi {
    Bar,
    Dots,
}

#[derive(Resource, Default)]
pub struct AchievementInfo {
    done_codes: Vec<String>,
    running_codes: Vec<String>,
}

impl AchievementInfo {
    pub fn reset(&mut self, store: &AchievementStore) {
        self.update_running_codes(store);
        self.done_codes = vec![];
    }

    pub fn update_running_codes(&mut self, store: &AchievementStore) {
        self.running_codes = vec![];
        for code in store.pinned_codes.iter() {
            if store.is_done(code) {
                continue;
            }
            self.running_codes.push(String::from(code));
            if self.running_codes.len() >= MAX_RUNNING_ACH_COUNT {
                return;
            }
        }
        for ach_def in ACHIEVEMENTS {
            if store.is_done(ach_def.code()) || store.is_pinned(ach_def.code()) {
                continue;
            }
            self.running_codes.push(String::from(ach_def.code()));
            if self.running_codes.len() >= MAX_RUNNING_ACH_COUNT {
                return;
            }
        }
    }

    pub fn is_running(&self, code: &str) -> bool {
        self.running_codes.contains(&String::from(code))
    }

    pub fn running_codes(&self) -> Vec<String> {
        self.running_codes.clone()
    }

    pub fn push_to_done(&mut self, code: &str) -> Option<String> {
        self.done_codes.insert(0, String::from(code));
        if self.done_codes.len() == 1 {
            self.done_codes.pop()
        } else {
            None
        }
    }

    pub fn next_done(&mut self) -> Option<String> {
        self.done_codes.pop()
    }
}

pub struct AchievementPlugin;

impl Plugin for AchievementPlugin {
    fn build(&self, app: &mut App) {
        let config_dir = dirs::config_dir()
            .map(|native_config_dir| native_config_dir.join("chain-reaction"))
            .unwrap_or(Path::new("local").join("configuration"));
        app.insert_resource(
            Persistent::<AchievementStore>::builder()
                .name("achievement")
                .format(StorageFormat::Bincode)
                .path(config_dir.join("achievement.bin"))
                .default(AchievementStore {
                    is_enabled: true,
                    pinned_codes: vec![],
                    record_map: HashMap::new(),
                })
                .build()
                .expect("failed to initialize variables"),
        );
        app.insert_resource(AchievementInfo::default());
    }
}

pub trait AchievementDefBase {
    fn code(&self) -> &str;
    fn name(&self) -> &str;
    fn color(&self) -> Color;
    fn icon(&self) -> &str {
        self.code()
    }
    fn description(&self) -> String;
    fn check_done(&self, status: &ResMut<status::ReactorStatus>) -> (u32, u32, bool);
    fn build_empty_record(&self) -> AchievementRecord {
        AchievementRecord {
            code: String::from(self.code()),
            is_done: false,
            done_at: String::from(""),
        }
    }
    fn icon_path(&self) -> String {
        format!("images/achievement/{}.png", self.icon())
    }
    fn progress_ui(&self) -> AchievementProgressUi;
    fn format_value(&self, value: u32) -> String {
        format!("{}", value)
    }
}

pub fn fetch_ach_def(code: &str) -> &dyn AchievementDefBase {
    for ach_def in ACHIEVEMENTS {
        if ach_def.code() == code {
            return ach_def;
        }
    }
    panic!("Invalid achievement code")
}
