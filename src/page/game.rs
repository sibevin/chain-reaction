use crate::{app, reactor};
use bevy::prelude::*;

pub struct PagePlugin;

impl Plugin for PagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(app::GameState::Game), page_setup)
            .add_systems(OnExit(app::GameState::Game), page_exit);
    }
}

fn page_setup(mut reactor_state: ResMut<NextState<reactor::ReactorState>>) {
    reactor_state.set(reactor::ReactorState::Ready);
}

fn page_exit(
    mut commands: Commands,
    particle_query: Query<Entity, With<reactor::RunningParticle>>,
    mut reactor_state: ResMut<NextState<reactor::ReactorState>>,
) {
    for entity in &particle_query {
        commands.entity(entity).despawn_recursive();
    }
    reactor_state.set(reactor::ReactorState::Demo);
}
