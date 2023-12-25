use crate::app::leaderboard::LeaderboardRecord;
use bevy::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq, Default)]
pub enum StatusChain {
    #[default]
    None,
    Hyper,
    Control,
}

#[derive(Resource, Default)]
pub struct ReactorStatus {
    pub player_name: String,
    pub highlight_uid: String,
    current_chain: StatusChain,
    chain_length: u32,
    time: u32,
    score: u32,
    alpha_count: u32,
    max_alpha_count: u32,
    max_control_chain: u32,
    max_hyper_chain: u32,
    created_ts: u64,
}

impl ReactorStatus {
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

    pub fn update_chain(&mut self, chain: StatusChain) {
        if self.current_chain == chain {
            match chain {
                StatusChain::Control => {
                    self.chain_length += 1;
                    self.compare_and_update_max_field("control_chain", self.chain_length)
                }
                StatusChain::Hyper => {
                    self.chain_length += 1;
                    self.compare_and_update_max_field("hyper_chain", self.chain_length)
                }
                StatusChain::None => (),
            }
        } else {
            match chain {
                StatusChain::Control => {
                    self.chain_length = 1;
                    self.compare_and_update_max_field("control_chain", self.chain_length)
                }
                StatusChain::Hyper => {
                    self.chain_length = 1;
                    self.compare_and_update_max_field("hyper_chain", self.chain_length)
                }
                StatusChain::None => {
                    self.chain_length = 0;
                }
            }
            self.current_chain = chain;
        }
    }

    pub fn export(&self) -> LeaderboardRecord {
        return LeaderboardRecord {
            player_name: self.player_name.clone(),
            time: self.time,
            score: self.score,
            max_alpha_count: self.max_alpha_count,
            max_control_chain: self.max_control_chain,
            max_hyper_chain: self.max_hyper_chain,
            created_ts: self.created_ts,
        };
    }

    pub fn mark_timestamp(&mut self) {
        self.created_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    pub fn reset(&mut self) {
        let highlight_uid = self.highlight_uid.clone();
        *self = self::default();
        self.highlight_uid = highlight_uid;
    }

    pub fn fetch(&self, field: &str) -> u32 {
        match field {
            "time" => {
                return self.time;
            }
            "score" => {
                return self.score;
            }
            "alpha_count" => {
                return self.alpha_count;
            }
            "chain_length" => {
                return self.chain_length;
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

    pub fn current_chain(&self) -> &StatusChain {
        &self.current_chain
    }

    pub fn increase(&mut self, field: &str, amount: u32) -> u32 {
        match field {
            "time" => {
                self.time += amount;
                return self.time;
            }
            "score" => {
                self.score += amount;
                return self.score;
            }
            "alpha_count" => {
                self.alpha_count += amount;
                return self.alpha_count;
            }
            _ => panic!("Invalid field"),
        }
    }

    pub fn update(&mut self, field: &str, value: u32) {
        match field {
            "alpha_count" => {
                self.alpha_count = value;
            }
            _ => panic!("Invalid field"),
        }
    }

    pub fn compare_and_update_max_field(&mut self, field: &str, value: u32) {
        match field {
            "alpha_count" => {
                if value > self.max_alpha_count {
                    self.max_alpha_count = value;
                }
            }
            "control_chain" => {
                if value > self.max_control_chain {
                    self.max_control_chain = value;
                }
            }
            "hyper_chain" => {
                if value > self.max_hyper_chain {
                    self.max_hyper_chain = value;
                }
            }
            _ => panic!("Invalid field"),
        }
    }
}
