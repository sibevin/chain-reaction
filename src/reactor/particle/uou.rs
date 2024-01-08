use crate::reactor::{field, particle::*};
use bevy_prototype_lyon::prelude::*;

pub const RADIUS: f32 = 8.0;
pub const COLOR: Color = Color::rgb(1.0, 0.84, 0.2);
const MIN_LEVEL: u8 = 1;
const MAX_LEVEL: u8 = 5;
const MIN_V: f32 = 0.0;
const MAX_V: f32 = 0.0;
const TAILING_SIZE: usize = 5;
const TAILING_WINDOW: u8 = 3;

pub struct Ability {
    tailings: CircularQueue<Vec2>,
    tailing_counter: u8,
}

impl Ability {
    pub fn gen_particle(
        pos: Vec2,
        direction: Option<Vec2>,
        level: Option<u8>,
        canvas_entity: Option<Entity>,
    ) -> Particle {
        Particle::new(
            Box::new(Ability {
                tailings: CircularQueue::with_capacity(TAILING_SIZE),
                tailing_counter: 0,
            }),
            pos,
            direction,
            level,
            canvas_entity,
        )
    }
}

impl ParticleAbility for Ability {
    fn particle_type(&self) -> ParticleType {
        ParticleType::Uou
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
    fn gen_random_v(&self, _direction: Option<Vec2>) -> Vec2 {
        Vec2::new(0.0, 0.0)
    }
    fn tailings(&self) -> Option<&CircularQueue<Vec2>> {
        Some(&self.tailings)
    }
    fn record_tailing(&mut self, pos: Vec2) {
        if self.tailing_counter == 0 {
            self.tailings.push(pos);
            self.tailing_counter = TAILING_WINDOW;
        } else {
            self.tailing_counter -= 1;
        }
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
                    transform: Transform::from_xyz(0.0, 0.0, 0.2),
                    sprite: Sprite {
                        color: COLOR,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    let shape = shapes::Circle {
                        radius: RADIUS,
                        center: Vec2::new(0.0, 0.0),
                    };
                    parent.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            ..default()
                        },
                        Fill::color(COLOR),
                    ));
                });
            canvas_entity = Some(
                parent
                    .spawn(SpriteBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 0.1),
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
    let particle = Particle::create(ParticleType::Uou, pos, direction, level, canvas_entity);
    update_particle_sprite(commands, &particle);
    commands.entity(root_entity).insert(particle);
}

pub fn update_particle_sprite(commands: &mut Commands, particle: &Particle) {
    if let Some(entity) = particle.canvas_entity() {
        if let Some(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn_descendants();
            if let Some(tailings) = particle.tailings() {
                entity_commands.with_children(|parent| {
                    let mut path_builder = PathBuilder::new();
                    let mut next_pos = Vec2::default();
                    for tailing in tailings.iter() {
                        path_builder.move_to(next_pos);
                        next_pos = *tailing - particle.pos();
                        path_builder.line_to(next_pos);
                    }
                    parent.spawn((
                        ShapeBundle {
                            path: path_builder.build(),
                            ..default()
                        },
                        Stroke {
                            options: StrokeOptions::default()
                                .with_end_cap(LineCap::Round)
                                .with_start_cap(LineCap::Round)
                                .with_line_width(RADIUS * 2.0),
                            color: COLOR.with_l(0.1),
                        },
                    ));
                });
            }
        }
    }
}
