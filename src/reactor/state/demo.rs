use crate::{
    app,
    reactor::{self, anime_effect::*, field, field_ach, hit::*, particle::*, status},
};
use bevy::prelude::*;
use bevy_tweening::*;
use std::collections::HashSet;
use std::f32::consts::PI;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(reactor::ReactorState::Demo),
            (
                field::reset_reactor_fields,
                field::reset_target_fields,
                field_ach::reset_ach_fields,
                clear_anime_effect,
                state_setup,
            ),
        )
        .add_systems(
            Update,
            (
                state_action,
                field::update_reactor_fields,
                field::update_target_fields,
                handle_particle_reaction,
                component_animator_system::<Particle>,
                component_animator_system::<AnimeEffect>,
            )
                .run_if(in_state(reactor::ReactorState::Demo)),
        )
        .add_systems(OnExit(reactor::ReactorState::Demo), state_exit);
    }
}

#[derive(Component)]
struct DemoParticle;

fn state_setup(
    mut commands: Commands,
    mut key_binding: ResMut<app::key_binding::KeyBindingConfig>,
) {
    key_binding.mode = app::key_binding::KeyBindingMode::Navgation;
    trigger::build_particle_sprite(&mut commands, DemoParticle, None, None, None);
    trigger::build_particle_sprite(&mut commands, DemoParticle, None, None, None);
    trigger::build_particle_sprite(&mut commands, DemoParticle, None, None, None);
    hyper::build_particle_sprite(&mut commands, DemoParticle, None, None, None);
    hyper::build_particle_sprite(&mut commands, DemoParticle, None, None, None);
    hyper::build_particle_sprite(&mut commands, DemoParticle, None, None, None);
}

fn state_exit(mut commands: Commands, particle_query: Query<Entity, With<DemoParticle>>) {
    for entity in &particle_query {
        commands.entity(entity).despawn_recursive();
    }
}

fn state_action(
    mut commands: Commands,
    mut particle_query: Query<(&mut Transform, &mut Particle), With<Particle>>,
    mut reactor_timer: ResMut<reactor::ReactorTimer>,
    time: Res<Time>,
    status: ResMut<status::ReactorStatus>,
) {
    if reactor_timer.0.tick(time.delta()).just_finished() {
        let alpha_count = status.fetch("alpha_count");
        for (mut transform, mut particle) in particle_query.iter_mut() {
            if particle.is_traveling() {
                let new_pos = (*particle).travel();
                transform.translation.x = new_pos.x;
                transform.translation.y = new_pos.y;
            }
            match particle.particle_type() {
                ParticleType::Alpha => {
                    particle.tick_countdown();
                }
                ParticleType::Hyper => {
                    if particle.tick_countdown() == 0 {
                        if alpha_count > 150 {
                            let new_pos = field::gen_random_pos_in_field(particle.radius);
                            (*particle).jump(new_pos);
                            transform.translation.x = new_pos.x;
                            transform.translation.y = new_pos.y;
                            control::build_particle_sprite(
                                &mut commands,
                                DemoParticle,
                                None,
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
                }
                ParticleType::Trigger => {
                    transform.rotate_z(-time.delta_seconds() * 2.0);
                    trigger::update_particle_level(particle.as_mut(), alpha_count);
                    if particle.tick_countdown() == 0 {
                        particle.reset_countdown();
                        let (_, _, angle) = transform.rotation.to_euler(EulerRot::XYZ);
                        let angle = angle + std::f32::consts::PI * 0.5;
                        let direction = Vec2::new(angle.cos(), angle.sin());
                        let level = alpha::pick_random_alpha_level();
                        alpha::build_particle_sprite(
                            &mut commands,
                            DemoParticle,
                            Some(particle.pos() + direction * particle.radius),
                            Some(direction),
                            Some(level),
                        );
                        if level > 2 {
                            insert_anime_effect(
                                &mut commands,
                                AnimeEffectParam {
                                    kind: AnimeEffectKind::Explosion,
                                    shape: AnimeEffectShape::Triangle,
                                    start_pos: particle.pos(),
                                    end_pos: particle.pos(),
                                },
                            );
                        }
                    }
                }
                _ => (),
            }
        }
    }
}

fn handle_particle_reaction(
    mut commands: Commands,
    mut particle_query: Query<(Entity, &mut Particle, &mut Transform), With<Particle>>,
    ae_query: Query<(Entity, &mut AnimeEffect), With<AnimeEffect>>,
    mut reactor_timer: ResMut<reactor::ReactorTimer>,
    mut painter_timer: ResMut<reactor::PainterTimer>,
    mut tween_completed_events: EventReader<TweenCompleted>,
    time: Res<Time>,
) {
    if painter_timer.0.tick(time.delta()).just_finished() {
        for (_, particle, _) in particle_query.iter() {
            particle.state_update(&mut commands);
        }
        for (_, mut particle, _) in particle_query.iter_mut() {
            if particle.state == ParticleState::Created {
                particle.state_setup(&mut commands);
            }
        }
        for (_, ae) in ae_query.iter() {
            update_anime_effect(&mut commands, ae);
        }
    }
    if reactor_timer.0.tick(time.delta()).just_finished() {
        let hit_map = detect_hit(&mut particle_query);
        let mut entities_to_despawn: HashSet<Entity> = HashSet::new();
        for (e, mut p, _) in particle_query.iter_mut() {
            if let Some(action) = hit_map.get(&e) {
                match p.particle_type() {
                    ParticleType::Alpha => match action {
                        HitAction::Kill => {
                            alpha::setup_particle_ending(&mut commands, &mut p);
                        }
                        HitAction::Release(count) => {
                            p.reset_countdown();
                            if *count > 1 {
                                for i in 1..=*count {
                                    let angle = PI * 2.0 * ((i - 1) as f32 + 0.25) / *count as f32;
                                    let direction = Vec2::new(angle.cos(), angle.sin());
                                    alpha::build_particle_sprite(
                                        &mut commands,
                                        DemoParticle,
                                        Some(p.pos() + direction * p.radius * 3.0),
                                        Some(direction),
                                        Some(1),
                                    );
                                }
                            } else {
                                alpha::build_particle_sprite(
                                    &mut commands,
                                    DemoParticle,
                                    Some(p.pos()),
                                    None,
                                    None,
                                );
                            }
                            if *count > 3 {
                                insert_anime_effect(
                                    &mut commands,
                                    AnimeEffectParam {
                                        kind: AnimeEffectKind::Explosion,
                                        shape: AnimeEffectShape::Circle,
                                        start_pos: p.pos(),
                                        end_pos: p.pos(),
                                    },
                                );
                            }
                            entities_to_despawn.insert(e);
                        }
                        HitAction::MoveOnly => {
                            p.reset_countdown();
                            p.assign_random_v(None);
                        }
                        _ => (),
                    },
                    ParticleType::Control => match action {
                        HitAction::AlphaHit(count) => {
                            for _ in 1..=*count {
                                if p.tick_countdown() == 0 {
                                    control::setup_particle_ending(&mut commands, &mut p);
                                }
                            }
                        }
                        HitAction::Kill => {
                            control::setup_particle_ending(&mut commands, &mut p);
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
        for tween_event in tween_completed_events.read() {
            if tween_event.user_data == STARTING_DONE_EVENT {
                for (e, mut p, _) in particle_query.iter_mut() {
                    if e == tween_event.entity {
                        p.state_starting_done(&mut commands);
                    }
                }
            }
            if tween_event.user_data == ENDING_DONE_EVENT
                || tween_event.user_data == ANIME_EFFECT_DONE_EVENT
            {
                entities_to_despawn.insert(tween_event.entity);
            }
        }
        for entity in entities_to_despawn.iter() {
            if let Some(entity_commands) = commands.get_entity(*entity) {
                entity_commands.despawn_recursive()
            }
        }
    }
}
