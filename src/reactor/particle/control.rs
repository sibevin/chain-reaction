use crate::{
    app,
    reactor::{field, particle::*},
};
use bevy_vector_shapes::prelude::*;
use std::f32::consts::{PI, TAU};

pub const COLOR: Color = Color::LIME_GREEN;
const MIN_LEVEL: u8 = 1;
const MAX_LEVEL: u8 = 8;
const MIN_V: f32 = 2.0;
const MAX_V: f32 = 4.0;
const RADIUS: f32 = 12.0;

pub struct Ability {
    countdown: u32,
}

impl Ability {
    pub fn gen_particle(pos: Vec2, direction: Option<Vec2>, level: Option<u8>) -> Particle {
        let mut particle = Particle::new(Box::new(Ability { countdown: 0 }), pos, direction, level);
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
    let particle = Particle::create(ParticleType::Control, pos, direction, level);
    let level_ratio = particle.level_ratio();
    let side_ratio = particle.countdown_ratio();
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
    update_particle_sprite(commands, root_entity, level_ratio, side_ratio);
}

pub fn update_particle_sprite(
    commands: &mut Commands,
    root_entity: Entity,
    level_ratio: f32,
    side_ratio: f32,
) {
    commands.entity(root_entity).despawn_descendants();
    commands.entity(root_entity).with_children(|parent| {
        parent.spawn(ShapeBundle::ngon(
            &ShapeConfig {
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                color: COLOR,
                ..ShapeConfig::default_2d()
            },
            4.0,
            RADIUS * 2.0_f32.sqrt(),
        ));
        let transform = Transform {
            translation: Vec3::new(0.0, 0.0, 2.0),
            rotation: Quat::from_rotation_z(PI * 0.1),
            ..default()
        };
        parent.spawn(ShapeBundle::ngon(
            &ShapeConfig {
                transform,
                color: app::ui::BG_COLOR,
                ..ShapeConfig::default_2d()
            },
            MAX_LEVEL as f32,
            RADIUS * 0.8,
        ));
        // let start_angle = -PI * 0.5 / MAX_LEVEL as f32;
        let start_angle = 0.0_f32;
        parent.spawn(ShapeBundle::arc(
            &ShapeConfig {
                transform: Transform::from_xyz(0.0, 0.0, 3.0),
                hollow: false,
                cap: Cap::None,
                color: COLOR,
                ..ShapeConfig::default_2d()
            },
            RADIUS * 0.9,
            start_angle,
            start_angle + TAU * level_ratio,
        ));
        for i in 1..=MAX_LEVEL {
            let angle = PI * 2.0 * (i + 1) as f32 / MAX_LEVEL as f32;
            parent.spawn(ShapeBundle::line(
                &ShapeConfig {
                    transform: Transform::from_xyz(0.0, 0.0, 4.0),
                    color: COLOR,
                    cap: Cap::None,
                    thickness: SIDE_THICKNESS * 3.0,
                    ..ShapeConfig::default_2d()
                },
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(angle.cos(), angle.sin(), 0.0) * RADIUS * 0.9,
            ));
        }
        parent.spawn(ShapeBundle::arc(
            &ShapeConfig {
                transform: Transform::from_xyz(0.0, 0.0, 5.0),
                color: COLOR,
                hollow: true,
                thickness: SIDE_THICKNESS,
                ..ShapeConfig::default_2d()
            },
            RADIUS * 1.5,
            0.0,
            TAU * side_ratio,
        ));
    });
}
