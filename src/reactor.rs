use crate::app;
use bevy::prelude::*;

pub mod field;
pub mod hit;
pub mod particle;
pub mod state;
pub mod timer;
pub mod tmm;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum ReactorState {
    #[default]
    Demo,
    Running,
    Paused,
    Ended,
}

#[derive(Component, Deref, DerefMut)]
pub struct ReactorTimer(pub Timer);

pub const U_SIZE: f32 = app::ui::SPACE_SIZE * 3.0;
pub const U_COLOR: Color = Color::rgb(1.0, 0.84, 0.2);

pub const FIELD_NAV_H: f32 = 80.0;
pub const FIELD_W: f32 = app::WINDOW_W;
pub const FIELD_H: f32 = app::WINDOW_H - FIELD_NAV_H;
