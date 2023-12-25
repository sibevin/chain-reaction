use crate::reactor::{self, field, particle::*};
use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(reactor::ReactorState::Ready),
            (field::reset_reactor_field, state_setup),
        );
    }
}

fn state_setup(
    mut commands: Commands,
    particle_query: Query<Entity, With<reactor::RunningParticle>>,
    mut reactor_state: ResMut<NextState<reactor::ReactorState>>,
) {
    for entity in &particle_query {
        commands.entity(entity).despawn_recursive();
    }
    uou::build_particle_sprite(
        &mut commands,
        (reactor::RunningParticle, reactor::ControlParticle),
        Some(Vec2::new(0.0, 0.0)),
        None,
        None,
    );
    let hyper_pos = Vec2::new(reactor::FIELD_H / 3.0, -reactor::FIELD_H / 3.0);
    hyper::build_particle_sprite(
        &mut commands,
        reactor::RunningParticle,
        Some(hyper_pos),
        Some(hyper_pos),
        None,
    );
    let hyper_pos = Vec2::new(-reactor::FIELD_H / 3.0, -reactor::FIELD_H / 3.0);
    hyper::build_particle_sprite(
        &mut commands,
        reactor::RunningParticle,
        Some(hyper_pos),
        Some(hyper_pos),
        None,
    );
    let trigger_pos = Vec2::new(reactor::FIELD_H / 3.0, reactor::FIELD_H / 3.0);
    trigger::build_particle_sprite(
        &mut commands,
        reactor::RunningParticle,
        Some(trigger_pos),
        Some(trigger_pos),
        None,
    );
    let trigger_pos = Vec2::new(-reactor::FIELD_H / 3.0, reactor::FIELD_H / 3.0);
    trigger::build_particle_sprite(
        &mut commands,
        reactor::RunningParticle,
        Some(trigger_pos),
        Some(trigger_pos),
        None,
    );
    reactor_state.set(reactor::ReactorState::Running);
}
