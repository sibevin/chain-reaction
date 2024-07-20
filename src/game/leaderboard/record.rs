use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Serialize, Deserialize, Clone, Debug, Default)]
pub struct LeaderboardRecord {
    pub player_name: String,
    pub time: u32,
    pub score: u32,
    pub max_alpha_count: u32,
    pub max_control_chain: u32,
    pub max_hyper_chain: u32,
    pub total_control_count: u32,
    pub total_hyper_count: u32,
    pub max_control_count: u32,
    pub max_full_level_control_count: u32,
    pub max_control_level: u32,
    pub max_hyper_level: u32,
    pub total_stopping_time: u32,
    pub max_stopping_time: u32,
    pub started_at: String,
    pub ended_at: String,
}

impl LeaderboardRecord {
    pub fn uid(&self) -> &str {
        &self.started_at
    }

    pub fn fetch(&self, field: &str) -> u32 {
        match field {
            "time" => self.time,
            "score" => self.score,
            "max_alpha_count" => self.max_alpha_count,
            "max_control_chain" => self.max_control_chain,
            "max_hyper_chain" => self.max_hyper_chain,
            "total_control_count" => self.total_control_count,
            "total_hyper_count" => self.total_hyper_count,
            "max_control_count" => self.max_control_count,
            "max_full_level_control_count" => self.max_full_level_control_count,
            "max_control_level" => self.max_control_level,
            "max_hyper_level" => self.max_hyper_level,
            "max_stopping_time" => self.max_stopping_time,
            _ => panic!("Invalid field"),
        }
    }
}
