use crate::{
    app,
    reactor::{self, particle::*},
};
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

fn page_setup(mut commands: Commands, mut reactor_state: ResMut<NextState<reactor::ReactorState>>) {
    uou::build_particle_sprite(
        &mut commands,
        (reactor::RunningParticle, reactor::ControlParticle),
        Some(Vec2::new(0.0, 0.0)),
        None,
        None,
    );
    let hyper_pos = Vec2::new(reactor::FIELD_H / 3.0, reactor::FIELD_H / 3.0);
    hyper::build_particle_sprite(
        &mut commands,
        reactor::RunningParticle,
        Some(hyper_pos),
        Some(Particle::gen_random_v(Some(hyper_pos))),
        None,
    );
    let trigger_pos = Vec2::new(-reactor::FIELD_H / 3.0, reactor::FIELD_H / 3.0);
    trigger::build_particle_sprite(
        &mut commands,
        reactor::RunningParticle,
        Some(trigger_pos),
        Some(Particle::gen_random_v(Some(trigger_pos))),
        None,
    );
    reactor_state.set(reactor::ReactorState::Running)
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
