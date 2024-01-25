use crate::reactor::{field, particle::*};
use bevy_prototype_lyon::prelude::*;
use bevy_tweening::*;
use circular_queue::CircularQueue;
use std::f32::consts::PI;
use std::time::Duration;

pub const RADIUS: f32 = 6.0;
pub const COLOR: Color = Color::SILVER;
const MIN_LEVEL: u8 = 1;
const MAX_LEVEL: u8 = 5;
const MIN_V: f32 = 0.3;
const MAX_V: f32 = 1.0;
const MAX_COUNTDOWN: u32 = 300;
const TAILING_SIZE: usize = 5;
const TAILING_WINDOW: u8 = 3;

pub struct Ability {
    countdown: u32,
    tailings: CircularQueue<Vec2>,
    tailing_counter: u8,
}

impl Ability {
    pub fn gen_particle(
        pos: Vec2,
        direction: Option<Vec2>,
        level: Option<u8>,
        root_entity: Entity,
        canvas_entity: Entity,
    ) -> Particle {
        let level = level.unwrap_or(pick_random_alpha_level());
        let mut particle = Particle::new(
            Box::new(Ability {
                countdown: 0,
                tailings: CircularQueue::with_capacity(TAILING_SIZE),
                tailing_counter: 0,
            }),
            pos,
            direction,
            Some(level),
            root_entity,
            canvas_entity,
        );
        particle.reset_countdown();
        particle
    }
}

impl ParticleAbility for Ability {
    fn particle_type(&self) -> ParticleType {
        ParticleType::Alpha
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
    fn max_countdown(&self, _: u8) -> u32 {
        MAX_COUNTDOWN
    }
    fn reset_countdown(&mut self, _: u8) {
        self.countdown = MAX_COUNTDOWN;
    }
    fn tick_countdown(&mut self) -> u32 {
        if self.countdown > 0 {
            self.countdown -= 1;
        }
        self.countdown
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
    fn is_traveling(&self, particle: &Particle) -> bool {
        particle.state == ParticleState::Running
    }
    fn state_setup(&self, commands: &mut Commands, particle: &Particle) -> ParticleState {
        setup_particle_running(commands, particle)
    }
    fn state_update(&self, commands: &mut Commands, particle: &Particle) {
        update_particle_ending(commands, particle);
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
                    transform: Transform::from_xyz(0.0, 0.0, 0.1),
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
        ParticleType::Alpha,
        pos,
        direction,
        level,
        root_entity,
        canvas_entity,
    );
    commands.entity(root_entity).insert(particle);
}

pub fn setup_particle_running(commands: &mut Commands, particle: &Particle) -> ParticleState {
    if let Some(mut entity_commands) = commands.get_entity(particle.root_entity()) {
        entity_commands.with_children(|parent| {
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
            if particle.level() > 1 {
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

                        for i in 1..=particle.level() {
                            let angle =
                                PI * 2.0 * ((i - 1) as f32 + 0.25) / particle.level() as f32;
                            path_builder.move_to(Vec2::default());
                            path_builder
                                .line_to(Vec2::new(angle.cos(), angle.sin()) * RADIUS * 1.5);
                        }

                        parent.spawn((
                            ShapeBundle {
                                path: path_builder.build(),
                                ..default()
                            },
                            Stroke::new(COLOR, SIDE_THICKNESS),
                        ));
                    });
            }
        });
    }
    ParticleState::Running
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
                    let shape = shapes::Circle {
                        radius: particle.radius,
                        center: Vec2::new(0.0, 0.0),
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
            Duration::from_millis(500),
            ParticleAnimeLens {
                start_radius: RADIUS,
                start_color_alpha: 0.3,
                end_radius: RADIUS * 5.0,
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
                    let shape = shapes::Circle {
                        radius: particle.radius,
                        center: Vec2::new(0.0, 0.0),
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

pub fn pick_random_alpha_level() -> u8 {
    let mut rng = thread_rng();
    let pick = rng.gen_range(0.0..100.0);
    if (0.0..=50.0).contains(&pick) {
        1
    } else if pick > 50.0 && pick <= 75.0 {
        2
    } else if pick > 75.0 && pick <= 87.0 {
        3
    } else if pick > 87.0 && pick <= 95.0 {
        4
    } else {
        5
    }
}
