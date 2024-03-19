use super::*;

pub struct ElementPlugin;

impl Plugin for ElementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(timer::ElementRefreshTimer(Timer::from_seconds(
            timer::ELEMENT_REFRESH_FRAME_SECS,
            TimerMode::Repeating,
        )))
        .insert_resource(timer::ElementBuildTimer(Timer::from_seconds(
            timer::ELEMENT_BUILD_DELAY_SECS,
            TimerMode::Once,
        )))
        .insert_resource(timer::ElementThrottleTimer(Timer::from_seconds(
            timer::ELEMENT_THROTTLE_SECS,
            TimerMode::Repeating,
        )))
        .add_event::<ElementEvent>();
    }
}
