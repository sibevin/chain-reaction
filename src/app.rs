use bevy::prelude::*;

pub mod audio;
pub mod init;
pub mod interaction;
pub mod key_binding;
pub mod leaderboard;
pub mod screenshot;
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
    Leaderboard,
    Auto,
}

pub const WINDOW_W: f32 = 1280.0;
pub const WINDOW_H: f32 = 720.0;
