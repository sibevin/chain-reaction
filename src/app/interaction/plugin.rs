use crate::app::interaction::*;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(default_focus::DelayTimer(Timer::from_seconds(
            default_focus::DELAY_TIMER_SECS,
            TimerMode::Once,
        )))
        .add_systems(
            Update,
            (
                handle::handle_button_interaction,
                handle::handle_menu_entry_interaction,
                handle::handle_switch_interaction,
                handle::handle_slider_interaction,
                handle::handle_link_interaction,
                handle::handle_cross_panel_interaction,
            )
                .after(NavRequestSystem),
        );
    }
}
