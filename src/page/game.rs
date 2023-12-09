use crate::{app, reactor};
use bevy::prelude::*;

pub struct PagePlugin;

impl Plugin for PagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(app::GameState::Game), page_setup)
            .add_systems(OnExit(app::GameState::Game), (reset_timer, page_exit))
            .add_plugins((
                reactor::state::running::StatePlugin,
                reactor::state::paused::StatePlugin,
            ));
    }
}

fn page_setup(mut commands: Commands, mut reactor_state: ResMut<NextState<reactor::ReactorState>>) {
    commands.spawn(reactor::ReactorTimer(Timer::from_seconds(
        0.01,
        TimerMode::Repeating,
    )));
    reactor_state.set(reactor::ReactorState::Running)
}

fn page_exit(mut reactor_state: ResMut<NextState<reactor::ReactorState>>) {
    reactor_state.set(reactor::ReactorState::Demo);
}

fn reset_timer(
    mut field_timer_query: Query<(&mut Text, &mut reactor::FieldTimer), With<reactor::FieldTimer>>,
) {
    for (mut text, mut field_timer) in field_timer_query.iter_mut() {
        field_timer.0 = 0;
        text.sections[0].value = reactor::field::format_field_text("time", field_timer.0);
    }
}
