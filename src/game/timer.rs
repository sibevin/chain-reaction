use super::*;

#[derive(Resource)]
pub struct GameRefreshTimer(pub Timer);

#[derive(Resource)]
pub struct GameBuildTimer(pub Timer);

#[derive(Resource)]
pub struct GameThrottleTimer(pub Timer);

#[derive(Resource)]
pub struct GameScoreboardTimer(pub Timer);

pub const GAME_BUILD_DELAY_SECS: f32 = 0.05;
pub const GAME_REFRESH_FRAME_SECS: f32 = 0.01;
pub const GAME_THROTTLE_SECS: f32 = 0.2;
pub const GAME_SCOREBOARD_SECS: f32 = 0.01;
