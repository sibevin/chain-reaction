use crate::reactor::{field, particle::*};
use bevy_prototype_lyon::prelude::*;
use std::f32::consts::TAU;

const MIN_LEVEL: u8 = 1;
const MAX_LEVEL: u8 = 5;
const MIN_V: f32 = 0.5;
const MAX_V: f32 = 1.5;
const RADIUS: f32 = 12.0;
const COLOR: Color = Color::rgb(1.0, 0.39, 0.29);

pub struct Ability {
    countdown: u32,
}

impl Ability {
    pub fn gen_particle(
        pos: Vec2,
        direction: Option<Vec2>,
        level: Option<u8>,
        canvas_entity: Option<Entity>,
    ) -> Particle {
        let mut particle = Particle::new(
            Box::new(Ability { countdown: 0 }),
            pos,
            direction,
            level,
            canvas_entity,
        );
        particle.reset_countdown();
        particle
    }
}

impl ParticleAbility for Ability {
    fn particle_type(&self) -> ParticleType {
        ParticleType::Trigger
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
        2_u32.pow(level as u32) * 50
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
    let mut canvas_entity = None;
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
                        sides: 3,
                        feature: shapes::RegularPolygonFeature::Radius(RADIUS),
                        ..shapes::RegularPolygon::default()
                    };
                    parent.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            ..default()
                        },
                        Fill::color(COLOR),
                    ));
                    let shape = shapes::Circle {
                        radius: alpha::RADIUS,
                        center: Vec2::new(0.0, RADIUS * 1.5),
                    };
                    parent.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            ..default()
                        },
                        Stroke::new(COLOR, SIDE_THICKNESS),
                    ));
                });
            canvas_entity = Some(
                parent
                    .spawn(SpriteBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 0.3),
                        sprite: Sprite {
                            color: COLOR,
                            ..default()
                        },
                        ..default()
                    })
                    .id(),
            );
        })
        .id();
    let particle = Particle::create(ParticleType::Trigger, pos, direction, level, canvas_entity);
    update_particle_sprite(commands, &particle);
    commands.entity(root_entity).insert(particle);
}

pub fn update_particle_sprite(commands: &mut Commands, particle: &Particle) {
    if let Some(entity) = particle.canvas_entity() {
        if let Some(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn_descendants();
            entity_commands.with_children(|parent| {
                let side_ratio = particle.countdown_ratio();
                let mut path_builder = PathBuilder::new();
                path_builder.move_to(Vec2::new(0.0, RADIUS * 1.5));
                path_builder.arc(
                    Vec2::default(),
                    Vec2::new(RADIUS * 1.5, RADIUS * 1.5),
                    TAU * side_ratio,
                    0.0,
                );
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
}

const LEVEL_INIT_BIAS_COUNT: u32 = 30;

pub fn update_particle_level(particle: &mut Particle, total_alpha_count: u32) {
    let mut level = 0;
    if total_alpha_count < LEVEL_INIT_BIAS_COUNT + 2_u32.pow((MIN_LEVEL + 1) as u32) {
        level = MIN_LEVEL;
    }
    if level == 0 {
        for i in MIN_LEVEL..=MAX_LEVEL {
            if total_alpha_count >= LEVEL_INIT_BIAS_COUNT + 2_u32.pow((i + 1) as u32)
                && total_alpha_count < LEVEL_INIT_BIAS_COUNT + 2_u32.pow((i + 2) as u32)
            {
                level = i;
                break;
            }
        }
    }
    if level == 0 {
        level = MAX_LEVEL;
    }
    if particle.level() != level {
        particle.update_level(level as i32 - particle.level() as i32);
    }
}
