use super::*;

#[derive(Resource)]
pub struct ElementRefreshTimer(pub Timer);

#[derive(Resource)]
pub struct ElementBuildTimer(pub Timer);

#[derive(Resource)]
pub struct ElementThrottleTimer(pub Timer);

pub const ELEMENT_BUILD_DELAY_SECS: f32 = 0.05;
pub const ELEMENT_REFRESH_FRAME_SECS: f32 = 0.2;
pub const ELEMENT_THROTTLE_SECS: f32 = 0.2;
