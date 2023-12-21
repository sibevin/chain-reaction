use crate::reactor::{field, particle::*};
use bevy_vector_shapes::prelude::*;
use std::f32::consts::PI;

pub const RADIUS: f32 = 6.0;
const MIN_LEVEL: u8 = 1;
const MAX_LEVEL: u8 = 5;
const MIN_V: f32 = 0.5;
const MAX_V: f32 = 1.5;
const COLOR: Color = Color::SILVER;
const MAX_COUNTDOWN: u32 = 300;

pub struct Ability {
    countdown: u32,
}

impl Ability {
    pub fn gen_particle(pos: Vec2, direction: Option<Vec2>, level: Option<u8>) -> Particle {
        let level = level.unwrap_or(Ability::pick_random_alpha_level());
        let mut particle = Particle::new(
            Box::new(Ability { countdown: 0 }),
            pos,
            direction,
            Some(level),
        );
        particle.reset_countdown();
        particle
    }

    fn pick_random_alpha_level() -> u8 {
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
    let particle = Particle::create(ParticleType::Alpha, pos, direction, level);
    let level = particle.level();
    let root_entity = commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                ..Default::default()
            },
            bundle,
            particle,
        ))
        .id();
    update_particle_sprite(commands, root_entity, level);
}

pub fn update_particle_sprite(commands: &mut Commands, root_entity: Entity, level: u8) {
    commands.entity(root_entity).despawn_descendants();
    commands.entity(root_entity).with_children(|parent| {
        parent.spawn(ShapeBundle::circle(
            &ShapeConfig {
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                color: COLOR,
                hollow: false,
                ..ShapeConfig::default_2d()
            },
            RADIUS,
        ));
        if level > 1 {
            for i in 1..=level {
                let angle = PI * 2.0 * ((i - 1) as f32 + 0.25) / level as f32;
                parent.spawn(ShapeBundle::line(
                    &ShapeConfig {
                        transform: Transform::from_xyz(0.0, 0.0, 4.0),
                        color: COLOR,
                        cap: Cap::None,
                        thickness: SIDE_THICKNESS * 3.0,
                        ..ShapeConfig::default_2d()
                    },
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(angle.cos(), angle.sin(), 0.0) * RADIUS * 1.5,
                ));
            }
        }
    });
}
