use bevy::prelude::*;

mod done;
mod none;
mod paused;
mod preparing;
mod ready;
mod running;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum StudioState {
    #[default]
    None,
    Preparing,
    Ready,
    Running,
    Paused,
    Done,
}

pub trait StudioStateBase {
    fn state(&self) -> StudioState;
    fn build(&self, app: &mut App);
}

pub const STUDIO_STATES: [&dyn StudioStateBase; 6] = [
    &done::State,
    &none::State,
    &paused::State,
    &preparing::State,
    &ready::State,
    &running::State,
];
