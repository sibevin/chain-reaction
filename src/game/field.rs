use super::*;
use crate::app::{self, theme, ui};

mod achievement;
mod main;

pub use main::field_systems;

pub const FIELD_BAR_H: f32 = 120.0;
pub const FIELD_W: f32 = app::WINDOW_W;
pub const FIELD_H: f32 = app::WINDOW_H;
pub const FIELD_P: f32 = 12.0;
pub const FIELD_LINE_W: f32 = ui::SPACE_SIZE * 0.5;
pub const FIELD_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
pub const FIELD_TEXT_COLOR: Color = theme::FG_COLOR;
pub const FIELD_SB_TEXT_COLOR: Color = theme::SECONDARY_COLOR;
