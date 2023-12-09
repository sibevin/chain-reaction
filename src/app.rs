use bevy::prelude::*;

pub mod audio;
pub mod init;
pub mod settings;
pub mod ui;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Game,
    Settings,
    About,
    Help,
}

pub const WINDOW_W: f32 = 1280.0;
pub const WINDOW_H: f32 = 720.0;
