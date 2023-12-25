use bevy::prelude::*;
use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const MAX_PLAYER_NAME_LENGTH: usize = 12;
pub const MAX_RECORDS_PER_LIST: usize = 9;
const LEADERBOARD_LISTS: [&str; 5] = [
    "time",
    "score",
    "max_alpha_count",
    "max_control_chain",
    "max_hyper_chain",
];

#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct LeaderboardRecord {
    pub player_name: String,
    pub time: u32,
    pub score: u32,
    pub max_alpha_count: u32,
    pub max_control_chain: u32,
    pub max_hyper_chain: u32,
    pub created_ts: u64,
}

impl LeaderboardRecord {
    pub fn uid(&self) -> String {
        format!(
            "{}_{}_{}_{}_{}_{}",
            self.created_ts,
            self.time,
            self.score,
            self.max_alpha_count,
            self.max_control_chain,
            self.max_hyper_chain
        )
    }

    pub fn fetch(&self, field: &str) -> u32 {
        match field {
            "time" => {
                return self.time;
            }
            "score" => {
                return self.score;
            }
            "max_alpha_count" => {
                return self.max_alpha_count;
            }
            "max_control_chain" => {
                return self.max_control_chain;
            }
            "max_hyper_chain" => {
                return self.max_hyper_chain;
            }
            _ => panic!("Invalid field"),
        }
    }
}

#[derive(Resource, Serialize, Deserialize)]
pub struct Leaderboard {
    records: Vec<LeaderboardRecord>,
}

impl Leaderboard {
    pub fn store(&mut self, record: LeaderboardRecord) {
        use std::cmp::Reverse;
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
        if records.len() == 0 {
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
        return 0;
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
        return false;
    }
}

pub struct LeaderboardPlugin;

impl Plugin for LeaderboardPlugin {
    fn build(&self, app: &mut App) {
        let config_dir = dirs::config_dir()
            .map(|native_config_dir| native_config_dir.join("chain-reaction"))
            .unwrap_or(Path::new("local").join("configuration"));

        app.insert_resource(
            Persistent::<Leaderboard>::builder()
                .name("leaderboard")
                .format(StorageFormat::Bincode)
                .path(config_dir.join("leaderboard.bin"))
                .default(Leaderboard { records: vec![] })
                .build()
                .expect("failed to initialize variables"),
        );
        app.insert_resource(LeaderboardRecord {
            player_name: String::from(""),
            time: 0,
            score: 0,
            max_alpha_count: 0,
            max_control_chain: 0,
            max_hyper_chain: 0,
            created_ts: 0,
        });
    }
}
