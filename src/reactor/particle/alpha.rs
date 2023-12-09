use crate::reactor::{field, particle::*};
use bevy_vector_shapes::prelude::*;

pub const RADIUS: f32 = 6.0;
const MIN_LEVEL: u8 = 1;
const MAX_LEVEL: u8 = 5;
const COLOR: Color = Color::SILVER;

pub struct Ability;

impl Ability {
    pub fn gen_particle(pos: Vec2, v: Option<Vec2>, level: Option<u8>) -> Particle {
        Particle::new(Box::new(Ability), pos, v, level)
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
}

pub fn build_particle_sprite(
    commands: &mut Commands,
    bundle: impl Bundle,
    pos: Option<Vec2>,
    v: Option<Vec2>,
    level: Option<u8>,
) {
    let pos = match pos {
        Some(pos) => pos,
        None => field::gen_random_pos_in_field(RADIUS * 2.0),
    };
    let particle = Particle::create(ParticleType::Alpha, pos, v, level);
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                ..Default::default()
            },
            bundle,
            particle,
        ))
        .with_children(|parent| {
            parent.spawn(ShapeBundle::circle(
                &ShapeConfig {
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    color: COLOR,
                    hollow: false,
                    ..ShapeConfig::default_2d()
                },
                RADIUS,
            ));
        });
}
