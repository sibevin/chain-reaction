use super::*;
use bevy::utils::HashMap;
use chrono::Local;
use serde::{Deserialize, Serialize};

const MAX_RUNNING_ACH_COUNT: usize = 2;

#[derive(Resource, Serialize, Deserialize)]
pub struct AchievementStore {
    pub is_enabled: bool,
    pub pinned_codes: Vec<String>,
    pub record_map: HashMap<String, AchievementRecord>,
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
