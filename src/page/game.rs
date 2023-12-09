use crate::{app, reactor};
use bevy::prelude::*;

pub struct PagePlugin;

impl Plugin for PagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(app::GameState::Game), page_setup)
            .add_systems(OnExit(app::GameState::Game), page_exit)
            .add_plugins((
                reactor::state::running::StatePlugin,
                reactor::state::paused::StatePlugin,
            ));
    }
}

fn page_setup(mut reactor_state: ResMut<NextState<reactor::ReactorState>>) {
    reactor_state.set(reactor::ReactorState::Running)
}

fn page_exit(mut reactor_state: ResMut<NextState<reactor::ReactorState>>) {
    reactor_state.set(reactor::ReactorState::Demo);
}
