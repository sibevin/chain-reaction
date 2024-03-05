use crate::app::interaction::*;
use bevy_ui_navigation::prelude::*;

pub const DELAY_TIMER_SECS: f32 = 0.2;

#[derive(Component)]
pub struct IaDefaultFocus;

#[derive(Resource)]
pub struct DelayTimer(pub Timer);

pub fn reset_default_focus(mut delay_timer: ResMut<DelayTimer>) {
    delay_timer.0.reset();
}

pub fn handle_default_focus(
    mut delay_timer: ResMut<DelayTimer>,
    time: Res<Time>,
    mut requests: EventWriter<NavRequest>,
    ia_default_focus: Query<Entity, With<IaDefaultFocus>>,
) {
    if delay_timer.0.tick(time.delta()).just_finished() {
        if let Ok(entity) = ia_default_focus.get_single() {
            requests.send(NavRequest::FocusOn(entity));
        }
    }
}
