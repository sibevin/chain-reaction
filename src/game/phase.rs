use super::*;

mod demo;
mod game_over;
mod paused;
mod ready;
mod running;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PhaseState {
    #[default]
    Demo,
    Ready,
    Running,
    Paused,
    GameOver,
}

pub trait PhaseBase {
    fn state(&self) -> PhaseState;
    fn build(&self, app: &mut App);
}

pub const PHASES: [&dyn PhaseBase; 5] = [
    &demo::Phase,
    &ready::Phase,
    &running::Phase,
    &paused::Phase,
    &game_over::Phase,
];
