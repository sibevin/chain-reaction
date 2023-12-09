use crate::{
    app,
    reactor::{self, field, particle::*},
};
use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(reactor::ReactorState::Demo),
            (
                state_setup,
                reactor::field::timer::reset_field,
                reactor::field::score::reset_field,
            ),
        )
        .add_systems(
            Update,
            (
                state_action,
                reactor::field::timer::update_field,
                reactor::field::alpha_count::update_field,
                reactor::field::score::update_field,
            )
                .run_if(in_state(reactor::ReactorState::Demo)),
        )
        .add_systems(
            OnExit(reactor::ReactorState::Demo),
            (
                state_exit,
                reactor::field::timer::reset_field,
                reactor::field::score::reset_field,
            ),
        );
    }
}

#[derive(Component)]
struct DemoParticle;

#[derive(Component)]
struct DemoCover;

fn state_setup(mut commands: Commands) {
    // for _ in 1..100 {
    //     alpha::build_particle_sprite(&mut commands, &particle_tmm, DemoParticle, None, None, None);
    // }
    hyper::build_particle_sprite(&mut commands, DemoParticle, None, None, None);
    trigger::build_particle_sprite(&mut commands, DemoParticle, None, None, None);
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            // background_color: app::ui::COVER_COLOR.into(),
            ..default()
        },
        DemoCover,
    ));
}

fn state_exit(
    mut commands: Commands,
    cover_query: Query<Entity, With<DemoCover>>,
    particle_query: Query<Entity, With<DemoParticle>>,
) {
    for entity in &particle_query {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &cover_query {
        commands.entity(entity).despawn_recursive();
    }
}

fn state_action(
    mut commands: Commands,
    mut particle_query: Query<(Entity, &mut Transform, &mut Particle), With<Particle>>,
    mut timer_query: Query<&mut reactor::ReactorTimer>,
    time: Res<Time>,
    alpha_count_query: Query<&field::FieldAlphaCount, With<field::FieldAlphaCount>>,
) {
    for mut timer in &mut timer_query {
        if timer.tick(time.delta()).just_finished() {
            let alpha_count = alpha_count_query.single().0;
            for (entity, mut transform, mut particle) in particle_query.iter_mut() {
                let new_pos = (*particle).travel();
                transform.translation.x = new_pos.x;
                transform.translation.y = new_pos.y;
                match particle.particle_type() {
                    ParticleType::Hyper => {
                        if particle.tick_countdown() == 0 {
                            if alpha_count > 10 {
                                let new_pos = field::gen_random_pos_in_field(particle.radius());
                                (*particle).jump(new_pos);
                                transform.translation.x = new_pos.x;
                                transform.translation.y = new_pos.y;
                                control::build_particle_sprite(
                                    &mut commands,
                                    DemoParticle,
                                    Some(particle.pos()),
                                    None,
                                    Some(particle.level()),
                                );
                            }
                            if particle.level() == 1 {
                                particle.update_level(5);
                            } else {
                                particle.update_level(-1);
                            }
                            particle.reset_countdown();
                        }
                        hyper::update_particle_sprite(
                            &mut commands,
                            entity,
                            particle.level_ratio(),
                            particle.countdown_ratio(),
                        );
                    }
                    ParticleType::Trigger => {
                        transform.rotate_z(-time.delta_seconds() * 2.0);
                        trigger::update_particle_level(particle.as_mut(), alpha_count);
                        if particle.tick_countdown() == 0 {
                            particle.reset_countdown();
                            let (_, _, angle) = transform.rotation.to_euler(EulerRot::XYZ);
                            let angle = angle + std::f32::consts::PI * 0.5;
                            let direction = Vec2::new(angle.cos(), angle.sin());
                            alpha::build_particle_sprite(
                                &mut commands,
                                DemoParticle,
                                Some(particle.pos() + direction * particle.radius()),
                                Some(Particle::gen_random_v(Some(direction))),
                                None,
                            );
                        }
                        trigger::update_particle_sprite(
                            &mut commands,
                            entity,
                            particle.countdown_ratio(),
                        );
                    }
                    _ => (),
                }
            }
        }
    }
}
