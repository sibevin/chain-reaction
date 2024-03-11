use crate::app::ui::*;

pub struct AppUiPlugin;

impl Plugin for AppUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(timer::AppUiRefreshTimer(Timer::from_seconds(
            timer::UI_REFRESH_FRAME_SECS,
            TimerMode::Repeating,
        )))
        .insert_resource(timer::AppUiBuildTimer(Timer::from_seconds(
            timer::UI_BUILD_DELAY_SECS,
            TimerMode::Once,
        )))
        .insert_resource(timer::AppUiThrottleTimer(Timer::from_seconds(
            timer::UI_THROTTLE_SECS,
            TimerMode::Repeating,
        )));
    }
}
