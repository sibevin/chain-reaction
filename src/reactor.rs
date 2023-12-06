use crate::app;
use bevy::prelude::*;

pub mod field;
pub mod state;
pub mod timer;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum ReactorState {
    #[default]
    Demo,
    Running,
    Paused,
    Ended,
}

#[derive(Component)]
pub struct FieldTimer(pub i32);

#[derive(Component)]
pub struct FieldAlphaCount(pub i32);

#[derive(Component)]
pub struct FieldScore(pub i32);

#[derive(Component, Deref, DerefMut)]
pub struct ReactorTimer(pub Timer);

pub const U_SIZE: f32 = app::ui::SPACE_SIZE * 3.0;
pub const U_COLOR: Color = Color::rgb(1.0, 0.84, 0.2);
pub const FIELD_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

pub const FIELD_NAV_H: f32 = 80.0;
pub const FIELD_W: f32 = app::WINDOW_W;
pub const FIELD_H: f32 = app::WINDOW_H - FIELD_NAV_H;
