use crate::{
    app,
    reactor::{field, particle::*},
};
use bevy_prototype_lyon::prelude::*;
use std::f32::consts::{PI, TAU};

pub const COLOR: Color = Color::rgb(0.1, 0.56, 1.0);
const MIN_LEVEL: u8 = 1;
const MAX_LEVEL: u8 = 6;
const MIN_V: f32 = 1.0;
const MAX_V: f32 = 3.0;
const RADIUS: f32 = 12.0;

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
        ParticleType::Hyper
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
        let countdown_pow = (MAX_LEVEL - level + 1) as u32;
        2_u32.pow(countdown_pow) * 15
    }
    fn reset_countdown(&mut self, level: u8) {
        self.countdown = self.max_countdown(level);
    }
    fn tick_countdown(&mut self) -> u32 {
        if self.countdown - 1 <= 0 {
            self.countdown = 0;
        } else {
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
                ..Default::default()
            },
            bundle,
        ))
        .with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.1),
                    ..default()
                })
                .with_children(|parent| {
                    let shape = shapes::RegularPolygon {
                        sides: 6,
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
                });
            parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.2),
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
                        Fill::color(app::ui::BG_COLOR),
                    ));
                });
            parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.3),
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
            canvas_entity = Some(
                parent
                    .spawn(SpriteBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 0.4),
                        ..default()
                    })
                    .id(),
            );
        })
        .id();
    let particle = Particle::create(ParticleType::Hyper, pos, direction, level, canvas_entity);
    update_particle_sprite(commands, &particle);
    commands.entity(root_entity).insert(particle);
}

pub fn update_particle_sprite(commands: &mut Commands, particle: &Particle) {
    if let Some(entity) = particle.canvas_entity() {
        if let Some(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn_descendants();
            entity_commands.with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 0.4),
                        ..default()
                    })
                    .with_children(|parent| {
                        let level_ratio = particle.level_ratio();
                        let level_angle = TAU * level_ratio;
                        let mut path_builder = PathBuilder::new();
                        path_builder.move_to(Vec2::new(0.0, 0.0));
                        let start_angle = TAU * 4.0 / 3.0 - level_angle;
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
                        if particle.level() > MIN_LEVEL {
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
                        }
                    });
            });
        }
    }
}
