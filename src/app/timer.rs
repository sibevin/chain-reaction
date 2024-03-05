use bevy::prelude::*;

const APP_FRAME_SECS: f32 = 0.01;

#[derive(Resource)]
pub struct AppTimer(pub Timer);

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AppTimer(Timer::from_seconds(
            APP_FRAME_SECS,
            TimerMode::Repeating,
        )));
    }
}
