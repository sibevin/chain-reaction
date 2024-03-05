use crate::{
    app::theme,
    reactor::{field, particle::*},
};
use bevy_prototype_lyon::prelude::*;
use bevy_tweening::*;
use std::f32::consts::{PI, TAU};
use std::time::Duration;

pub const COLOR: Color = Color::LIME_GREEN;
pub const MAX_LEVEL: u8 = 8;
const MIN_LEVEL: u8 = 1;
const MIN_V: f32 = 2.0;
const MAX_V: f32 = 4.0;
const RADIUS: f32 = 12.0;

pub struct Ability {
    countdown: u32,
}

impl Ability {
    pub fn gen_particle(
        pos: Vec2,
        direction: Option<Vec2>,
        level: Option<u8>,
        root_entity: Entity,
        canvas_entity: Entity,
    ) -> Particle {
        let mut particle = Particle::new(
            Box::new(Ability { countdown: 0 }),
            pos,
            direction,
            level,
            root_entity,
            canvas_entity,
        );
        particle.reset_countdown();
        particle
    }
}

impl ParticleAbility for Ability {
    fn particle_type(&self) -> ParticleType {
        ParticleType::Control
    }
    fn radius(&self) -> f32 {
        RADIUS
    }
    fn color(&self) -> Color {
        COLOR
    }
    fn min_level(&self) -> u8 {
        MIN_LEVEL
    }
    fn max_level(&self) -> u8 {
        MAX_LEVEL
    }
    fn min_v(&self) -> f32 {
        MIN_V
    }
    fn max_v(&self) -> f32 {
        MAX_V
    }
    fn current_countdown(&self) -> u32 {
        self.countdown
    }
    fn max_countdown(&self, level: u8) -> u32 {
        2_u32.pow(level as u32)
    }
    fn reset_countdown(&mut self, level: u8) {
        self.countdown = self.max_countdown(level);
    }
    fn tick_countdown(&mut self) -> u32 {
        if self.countdown > 0 {
            self.countdown -= 1;
        }
        self.countdown
    }
    fn is_traveling(&self, particle: &Particle) -> bool {
        particle.state != ParticleState::Starting
    }
    fn state_setup(&self, commands: &mut Commands, particle: &Particle) -> ParticleState {
        setup_particle_starting(commands, particle)
    }
    fn state_update(&self, commands: &mut Commands, particle: &Particle) {
        update_particle_starting(commands, particle);
        update_particle_running(commands, particle);
        update_particle_ending(commands, particle);
    }
    fn state_starting_done(&self, commands: &mut Commands, particle: &Particle) -> ParticleState {
        setup_particle_running(commands, particle)
    }
}

pub fn build_particle_sprite(
    commands: &mut Commands,
    bundle: impl Bundle,
    pos: Option<Vec2>,
    direction: Option<Vec2>,
    level: Option<u8>,
) {
    let pos = match pos {
        Some(pos) => pos,
        None => field::gen_random_pos_in_field(RADIUS * 2.0),
    };
    let mut canvas_entity: Entity = Entity::PLACEHOLDER;
    let root_entity = commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                sprite: Sprite {
                    color: COLOR,
                    ..default()
                },
                ..default()
            },
            bundle,
        ))
        .with_children(|parent| {
            canvas_entity = parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.4),
                    sprite: Sprite {
                        color: COLOR,
                        ..default()
                    },
                    ..default()
                })
                .id();
        })
        .id();
    let particle = Particle::create(
        ParticleType::Control,
        pos,
        direction,
        level,
        root_entity,
        canvas_entity,
    );
    commands.entity(root_entity).insert(particle);
}

pub fn setup_particle_starting(commands: &mut Commands, particle: &Particle) -> ParticleState {
    if let Some(mut entity_commands) = commands.get_entity(particle.root_entity()) {
        let tween = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_millis(800),
            ParticleAnimeLens {
                start_radius: RADIUS * 2.5,
                start_color_alpha: 0.0,
                end_radius: RADIUS,
                end_color_alpha: 0.5,
            },
        )
        .with_completed_event(STARTING_DONE_EVENT);
        entity_commands.insert(Animator::new(tween));
    }
    ParticleState::Starting
}

pub fn update_particle_starting(commands: &mut Commands, particle: &Particle) {
    if particle.state != ParticleState::Starting {
        return;
    }
    if let Some(mut entity_commands) = commands.get_entity(particle.canvas_entity()) {
        entity_commands.despawn_descendants();
        entity_commands.with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.1),
                    sprite: Sprite {
                        color: COLOR,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    let shape = shapes::RegularPolygon {
                        sides: 4,
                        feature: shapes::RegularPolygonFeature::Radius(
                            particle.radius * 2.0_f32.sqrt(),
                        ),
                        ..shapes::RegularPolygon::default()
                    };
                    parent.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            ..default()
                        },
                        Fill::color(particle.color),
                    ));
                });
        });
    }
}

pub fn setup_particle_running(commands: &mut Commands, particle: &Particle) -> ParticleState {
    if let Some(mut entity_commands) = commands.get_entity(particle.root_entity()) {
        entity_commands.with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.1),
                    sprite: Sprite {
                        color: COLOR,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    let shape = shapes::RegularPolygon {
                        sides: 4,
                        feature: shapes::RegularPolygonFeature::Radius(RADIUS * 2.0_f32.sqrt()),
                        ..shapes::RegularPolygon::default()
                    };
                    parent.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            ..default()
                        },
                        Fill::color(COLOR),
                    ));
                });
            parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.2),
                    sprite: Sprite {
                        color: COLOR,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    let shape = shapes::RegularPolygon {
                        sides: MAX_LEVEL as usize,
                        feature: shapes::RegularPolygonFeature::Radius(RADIUS * 0.7),
                        ..shapes::RegularPolygon::default()
                    };
                    parent.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            ..default()
                        },
                        Fill::color(theme::BG_COLOR),
                    ));
                });
            parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.3),
                    sprite: Sprite {
                        color: COLOR,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    let mut path_builder = PathBuilder::new();
                    for i in 1..=MAX_LEVEL {
                        let angle = PI * 2.0 * i as f32 / MAX_LEVEL as f32;
                        path_builder.move_to(Vec2::default());
                        path_builder.line_to(Vec2::new(angle.cos(), angle.sin()) * RADIUS * 0.8);
                    }
                    parent.spawn((
                        ShapeBundle {
                            path: path_builder.build(),
                            ..default()
                        },
                        Stroke::new(COLOR, SIDE_THICKNESS),
                    ));
                });
        });
    }
    ParticleState::Running
}

pub fn update_particle_running(commands: &mut Commands, particle: &Particle) {
    if particle.state != ParticleState::Running {
        return;
    }
    if let Some(mut entity_commands) = commands.get_entity(particle.canvas_entity()) {
        entity_commands.despawn_descendants();
        entity_commands.with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.4),
                    sprite: Sprite {
                        color: COLOR,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    let level_ratio = particle.level_ratio();
                    let level_angle = TAU * level_ratio;
                    let start_angle = PI * 2.5 - level_angle;
                    let mut path_builder = PathBuilder::new();
                    path_builder.move_to(Vec2::new(0.0, 0.0));
                    path_builder.line_to(Vec2::from_angle(start_angle) * RADIUS * 0.8);
                    path_builder.arc(
                        Vec2::default(),
                        Vec2::new(RADIUS * 0.8, RADIUS * 0.8),
                        level_angle,
                        0.0,
                    );
                    path_builder.close();
                    parent.spawn((
                        ShapeBundle {
                            path: path_builder.build(),
                            ..default()
                        },
                        Fill::color(COLOR),
                    ));
                    let side_ratio = particle.countdown_ratio();
                    let mut path_builder = PathBuilder::new();
                    path_builder.move_to(Vec2::new(0.0, RADIUS * 1.5));
                    path_builder.arc(
                        Vec2::default(),
                        Vec2::new(RADIUS * 1.5, RADIUS * 1.5),
                        -TAU * side_ratio,
                        0.0,
                    );
                    parent.spawn((
                        ShapeBundle {
                            path: path_builder.build(),
                            ..default()
                        },
                        Stroke::new(COLOR, SIDE_THICKNESS),
                    ));
                });
        });
    }
}

pub fn setup_particle_ending(commands: &mut Commands, particle: &mut Particle) {
    particle.state = ParticleState::Ending;
    if let Some(mut entity_commands) = commands.get_entity(particle.root_entity()) {
        entity_commands.despawn_descendants();
        entity_commands.with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.1),
                    sprite: Sprite {
                        color: COLOR,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    let shape = shapes::RegularPolygon {
                        sides: 4,
                        feature: shapes::RegularPolygonFeature::Radius(RADIUS * 2.0_f32.sqrt()),
                        ..shapes::RegularPolygon::default()
                    };
                    parent.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            ..default()
                        },
                        Fill::color(particle.color),
                    ));
                });
        });
        let tween = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_millis(300),
            ParticleAnimeLens {
                start_radius: RADIUS,
                start_color_alpha: 0.3,
                end_radius: RADIUS * 3.0,
                end_color_alpha: 0.0,
            },
        )
        .with_completed_event(ENDING_DONE_EVENT);
        entity_commands.insert(Animator::new(tween));
    }
}

pub fn update_particle_ending(commands: &mut Commands, particle: &Particle) {
    if particle.state != ParticleState::Ending {
        return;
    }
    if let Some(mut entity_commands) = commands.get_entity(particle.root_entity()) {
        entity_commands.despawn_descendants();
        entity_commands.with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.1),
                    sprite: Sprite {
                        color: COLOR,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    let shape = shapes::RegularPolygon {
                        sides: 4,
                        feature: shapes::RegularPolygonFeature::Radius(RADIUS * 2.0_f32.sqrt()),
                        ..shapes::RegularPolygon::default()
                    };
                    parent.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            ..default()
                        },
                        Fill::color(particle.color),
                    ));
                });
        });
    }
}
