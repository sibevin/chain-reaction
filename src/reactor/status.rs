use crate::app::leaderboard::LeaderboardRecord;
use bevy::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq, Default, Debug)]
pub enum StatusChain {
    #[default]
    None,
    Hyper,
    Control,
}

#[derive(Resource, Default, Debug)]
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
    total_control_count: u32,
    total_hyper_count: u32,
    max_control_count: u32,
    max_full_level_control_count: u32,
    max_control_level: u32,
    max_hyper_level: u32,
    current_stopping_time: u32,
    total_stopping_time: u32,
    max_stopping_time: u32,
    u_pos: Vec2,
    created_ts: u64,
}

impl ReactorStatus {
    pub fn uid(&self) -> String {
        format!("{}_{}_{}", self.created_ts, self.time, self.score,)
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
            total_control_count: self.total_control_count,
            total_hyper_count: self.total_hyper_count,
            max_control_count: self.max_control_count,
            max_full_level_control_count: self.max_full_level_control_count,
            max_control_level: self.max_control_level,
            max_hyper_level: self.max_hyper_level,
            total_stopping_time: self.total_stopping_time,
            max_stopping_time: self.max_stopping_time,
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
            "total_control_count" => {
                return self.total_control_count;
            }
            "total_hyper_count" => {
                return self.total_hyper_count;
            }
            "max_control_count" => {
                return self.max_control_count;
            }
            "max_full_level_control_count" => {
                return self.max_full_level_control_count;
            }
            "max_control_level" => {
                return self.max_control_level;
            }
            "max_hyper_level" => {
                return self.max_hyper_level;
            }
            "max_stopping_time" => {
                return self.max_stopping_time;
            }
            _ => panic!("Invalid field"),
        }
    }

    pub fn update_stopping_time(&mut self, new_u_pos: Vec2) {
        if self.current_stopping_time > self.max_stopping_time {
            self.max_stopping_time = self.current_stopping_time;
        }
        let is_stopping = self.u_pos == new_u_pos;
        self.u_pos = new_u_pos;
        if is_stopping {
            self.current_stopping_time += 1;
            self.total_stopping_time += 1;
        } else {
            self.current_stopping_time = 0;
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
            "total_control_count" => {
                self.total_control_count += amount;
                return self.total_control_count;
            }
            "total_hyper_count" => {
                self.total_hyper_count += amount;
                return self.total_hyper_count;
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
            "control_count" => {
                if value > self.max_control_count {
                    self.max_control_count = value;
                }
            }
            "full_level_control_count" => {
                if value > self.max_full_level_control_count {
                    self.max_full_level_control_count = value;
                }
            }
            "control_level" => {
                if value > self.max_control_level {
                    self.max_control_level = value;
                }
            }
            "hyper_level" => {
                if value > self.max_hyper_level {
                    self.max_hyper_level = value;
                }
            }
            "stopping_time" => {
                if value > self.max_stopping_time {
                    self.max_stopping_time = value;
                }
            }
            _ => panic!("Invalid field"),
        }
    }
}
