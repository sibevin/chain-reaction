use crate::app::leaderboard::LeaderboardRecord;
use bevy::prelude::*;
use chrono::Local;

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
    started_at: String,
    ended_at: String,
}

impl ReactorStatus {
    pub fn uid(&self) -> &str {
        &self.started_at
    }

    pub fn update_chain(&mut self, chain: StatusChain) {
        if self.current_chain == chain {
            match chain {
                StatusChain::Control => {
                    self.chain_length += 1;
                    self.compare_and_update_max_field("control_chain", self.chain_length);
                }
                StatusChain::Hyper => {
                    self.chain_length += 1;
                    self.compare_and_update_max_field("hyper_chain", self.chain_length);
                }
                StatusChain::None => (),
            }
        } else {
            match chain {
                StatusChain::Control => {
                    self.chain_length = 1;
                    self.compare_and_update_max_field("control_chain", self.chain_length);
                }
                StatusChain::Hyper => {
                    self.chain_length = 1;
                    self.compare_and_update_max_field("hyper_chain", self.chain_length);
                }
                StatusChain::None => {
                    self.chain_length = 0;
                }
            }
            self.current_chain = chain;
        }
    }

    pub fn export(&self) -> LeaderboardRecord {
        LeaderboardRecord {
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
            started_at: self.started_at.clone(),
            ended_at: self.ended_at.clone(),
        }
    }

    pub fn mark_timeline(&mut self, timeline_type: &str) {
        match timeline_type {
            "started" => {
                self.started_at = Local::now().format("%Y-%m-%d_%H:%M:%S%.9f").to_string();
            }
            "ended" => {
                self.ended_at = Local::now().format("%Y-%m-%d_%H:%M:%S%.9f").to_string();
            }
            _ => panic!("Invalid timeline type"),
        }
    }

    pub fn reset(&mut self) {
        let highlight_uid = self.highlight_uid.clone();
        *self = self::default();
        self.highlight_uid = highlight_uid;
        self.mark_timeline("started");
    }

    pub fn fetch(&self, field: &str) -> u32 {
        match field {
            "time" => {
                self.time
            }
            "score" => {
                self.score
            }
            "alpha_count" => {
                self.alpha_count
            }
            "chain_length" => {
                self.chain_length
            }
            "max_alpha_count" => {
                self.max_alpha_count
            }
            "max_control_chain" => {
                self.max_control_chain
            }
            "max_hyper_chain" => {
                self.max_hyper_chain
            }
            "total_control_count" => {
                self.total_control_count
            }
            "total_hyper_count" => {
                self.total_hyper_count
            }
            "max_control_count" => {
                self.max_control_count
            }
            "max_full_level_control_count" => {
                self.max_full_level_control_count
            }
            "max_control_level" => {
                self.max_control_level
            }
            "max_hyper_level" => {
                self.max_hyper_level
            }
            "max_stopping_time" => {
                self.max_stopping_time
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
                self.time
            }
            "score" => {
                self.score += amount;
                self.score
            }
            "alpha_count" => {
                self.alpha_count += amount;
                self.alpha_count
            }
            "total_control_count" => {
                self.total_control_count += amount;
                self.total_control_count
            }
            "total_hyper_count" => {
                self.total_hyper_count += amount;
                self.total_hyper_count
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

    pub fn compare_and_update_max_field(&mut self, field: &str, value: u32) -> bool {
        let mut is_updated = false;
        match field {
            "alpha_count" => {
                if value > self.max_alpha_count {
                    self.max_alpha_count = value;
                    is_updated = true;
                }
            }
            "control_chain" => {
                if value > self.max_control_chain {
                    self.max_control_chain = value;
                    is_updated = true;
                }
            }
            "hyper_chain" => {
                if value > self.max_hyper_chain {
                    self.max_hyper_chain = value;
                    is_updated = true;
                }
            }
            "control_count" => {
                if value > self.max_control_count {
                    self.max_control_count = value;
                    is_updated = true;
                }
            }
            "full_level_control_count" => {
                if value > self.max_full_level_control_count {
                    self.max_full_level_control_count = value;
                    is_updated = true;
                }
            }
            "control_level" => {
                if value > self.max_control_level {
                    self.max_control_level = value;
                    is_updated = true;
                }
            }
            "hyper_level" => {
                if value > self.max_hyper_level {
                    self.max_hyper_level = value;
                    is_updated = true;
                }
            }
            "stopping_time" => {
                if value > self.max_stopping_time {
                    self.max_stopping_time = value;
                    is_updated = true;
                }
            }
            _ => panic!("Invalid field"),
        }
        is_updated
    }
}
