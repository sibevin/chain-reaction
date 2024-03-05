use super::*;
use bevy::prelude::*;
use chrono::Local;

#[derive(Resource, Default, Debug)]
pub struct StudioStatus {
    pub uid: String,
    pub achievements: Vec<String>,
    pub property: StudioProperty,
    pub vase_color: Color,
    pub time: u32,
    pub score: u32,
    pub undo_count: u32,
    pub repaint_count: u32,
    pub match_point: u32,
    started_at: String,
    ended_at: String,
}

impl StudioStatus {
    pub fn reset(&mut self) {
        *self = self::default();
        self.mark_timeline("started");
        self.property = StudioProperty::generate();
    }

    pub fn mark_timeline(&mut self, timeline_type: &str) {
        let now_dt = Local::now().format("%Y-%m-%d_%H:%M:%S%.9f").to_string();
        match timeline_type {
            "started" => {
                self.started_at = now_dt;
            }
            "ended" => {
                self.ended_at = now_dt;
            }
            _ => panic!("Invalid timeline type"),
        }
    }
}
