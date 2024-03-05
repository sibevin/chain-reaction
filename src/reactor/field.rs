use crate::app;

use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub const FIELD_ACH_H: f32 = 120.0;
pub const FIELD_NAV_H: f32 = 80.0;
pub const FIELD_W: f32 = app::WINDOW_W;
pub const FIELD_H: f32 = app::WINDOW_H - FIELD_ACH_H - FIELD_NAV_H;

pub fn get_field_rect(padding: f32) -> Rect {
    Rect::new(
        -FIELD_W / 2.0 + padding,
        (-FIELD_H + FIELD_NAV_H - FIELD_ACH_H) / 2.0 + padding,
        FIELD_W / 2.0 - padding,
        (FIELD_H + FIELD_NAV_H - FIELD_ACH_H) / 2.0 - padding,
    )
}

pub fn gen_random_pos_in_field(padding: f32) -> Vec2 {
    let mut rng = thread_rng();
    let rect = get_field_rect(padding);
    Vec2::new(
        rng.gen_range(rect.min.x..rect.max.x),
        rng.gen_range(rect.min.y..rect.max.y),
    )
}
