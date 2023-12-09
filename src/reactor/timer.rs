use crate::reactor;
use bevy::prelude::*;

pub fn init_timer(mut commands: Commands) {
    commands.spawn(reactor::ReactorTimer(Timer::from_seconds(
        0.01,
        TimerMode::Repeating,
    )));
}
