use crate::reactor::{self, field, field_ach, particle::*};
use bevy::prelude::*;
use std::f32::consts::PI;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(reactor::ReactorState::Ready),
            (
                field::reset_reactor_fields,
                field::reset_target_fields,
                field_ach::reset_ach_fields,
                state_setup,
            ),
        );
    }
}

const PARTICLE_COUNT: u8 = 3;
const POS_RADIUS: f32 = reactor::FIELD_H / 3.0;

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
    let start_angle = 0.0;
    for i in 0..PARTICLE_COUNT {
        let angle = (start_angle + PI * 2.0 * i as f32) / PARTICLE_COUNT as f32;
        let pos = Vec2::new(POS_RADIUS * angle.cos(), POS_RADIUS * angle.sin());
        hyper::build_particle_sprite(
            &mut commands,
            reactor::RunningParticle,
            Some(pos),
            Some(pos),
            None,
        );
    }
    let start_angle = 60.0;
    for i in 0..PARTICLE_COUNT {
        let angle = (start_angle + PI * 2.0 * i as f32) / PARTICLE_COUNT as f32;
        let pos = Vec2::new(POS_RADIUS * angle.cos(), POS_RADIUS * angle.sin());
        trigger::build_particle_sprite(
            &mut commands,
            reactor::RunningParticle,
            Some(pos),
            Some(pos),
            None,
        );
    }
    reactor_state.set(reactor::ReactorState::Running);
}
