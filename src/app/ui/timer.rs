use crate::app::ui::*;

#[derive(Resource)]
pub struct AppUiRefreshTimer(pub Timer);

#[derive(Resource)]
pub struct AppUiBuildTimer(pub Timer);

#[derive(Resource)]
pub struct AppUiThrottleTimer(pub Timer);

pub const UI_BUILD_DELAY_SECS: f32 = 0.1;
pub const UI_REFRESH_FRAME_SECS: f32 = 0.2;
pub const UI_THROTTLE_SECS: f32 = 0.2;
