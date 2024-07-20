use super::record::LeaderboardRecord;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use crate::app;

pub const MAX_PLAYER_NAME_LENGTH: usize = 12;
pub const MAX_RECORDS_PER_LIST: usize = 9;
pub const LEADERBOARD_LISTS: [&str; 5] = [
    "score",
    "time",
    "max_alpha_count",
    "max_control_chain",
    "max_hyper_chain",
];

#[derive(Resource, Serialize, Deserialize)]
pub struct LeaderboardModel {
    pub records: Vec<LeaderboardRecord>,
}

impl LeaderboardModel {
    pub fn store(&mut self, record: LeaderboardRecord) {
        use std::cmp::Reverse;
        #[cfg(not(target_arch = "wasm32"))]
        app::screenshot::store_leaderboard_screenshots(record.uid());
        self.records.push(record);
        self.records.sort_by_key(|record| {
            (
                Reverse(record.score),
                Reverse(record.time),
                Reverse(record.max_alpha_count),
                Reverse(record.max_control_chain),
                Reverse(record.max_hyper_chain),
            )
        });
        if self.records.len() > MAX_RECORDS_PER_LIST * 5 {
            self.records.pop();
        }
    }

    pub fn fetch_records(&self, field: &str) -> Vec<LeaderboardRecord> {
        use std::cmp::Reverse;
        let mut records = self.records.clone();
        records.sort_by_key(|record| match field {
            "time" => (Reverse(record.time), Reverse(record.score)),
            "score" => (Reverse(record.score), Reverse(record.time)),
            "max_alpha_count" => (Reverse(record.max_alpha_count), Reverse(record.score)),
            "max_control_chain" => (Reverse(record.max_control_chain), Reverse(record.score)),
            "max_hyper_chain" => (Reverse(record.max_hyper_chain), Reverse(record.score)),
            _ => panic!("Invalid record field"),
        });
        records.into_iter().take(MAX_RECORDS_PER_LIST).collect()
    }

    pub fn rank(&self, field: &str, value: u32) -> u8 {
        let records = self.fetch_records(field);
        if records.is_empty() {
            return 1;
        }
        let mut list_rank = 1;
        let mut prev_value: u32 = 0;
        for i in 0..MAX_RECORDS_PER_LIST {
            if let Some(record) = records.get(i) {
                let list_value = record.fetch(field);
                if i == 0 {
                    list_rank = 1;
                    prev_value = list_value;
                } else if list_value < prev_value {
                    list_rank = i + 1;
                    prev_value = list_value;
                }
                if value >= list_value {
                    return list_rank as u8;
                }
            }
        }
        0
    }

    pub fn target(&self, field: &str, value: u32) -> (u8, u32, u32) {
        let records = self.fetch_records(field);
        if records.is_empty() {
            return (0, 0, 0);
        }
        let mut list_rank = 0;
        let mut prev_value: u32 = 0;
        for i in 0..MAX_RECORDS_PER_LIST {
            if let Some(record) = records.get(i) {
                let list_value = record.fetch(field);
                if i == 0 {
                    if value >= list_value {
                        return (0, 0, 0);
                    }
                    list_rank = 1;
                    prev_value = list_value;
                } else if list_value < prev_value {
                    if value >= list_value {
                        return (list_rank as u8, prev_value, list_value);
                    }
                    list_rank = i + 1;
                    prev_value = list_value;
                }
            }
        }
        (list_rank as u8, prev_value, 0)
    }

    pub fn is_new_in_list(&self, field: &str, value: u32) -> bool {
        let records = self.fetch_records(field);
        if records.len() < MAX_RECORDS_PER_LIST {
            return true;
        }
        let last_value = records[MAX_RECORDS_PER_LIST - 1].fetch(field);
        last_value < value
    }

    pub fn is_new_record(&self, record: &LeaderboardRecord) -> bool {
        for field in LEADERBOARD_LISTS {
            if self.is_new_in_list(field, record.fetch(field)) {
                return true;
            }
        }
        false
    }
}
