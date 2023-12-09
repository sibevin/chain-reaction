use crate::reactor::{field, particle::*, tmm::*};
use bevy_vector_shapes::prelude::*;
use std::f32::consts::TAU;

const MIN_LEVEL: u8 = 1;
const MAX_LEVEL: u8 = 8;
const RADIUS: f32 = 12.0;
const COLOR: Color = Color::rgb(1.0, 0.39, 0.29);

pub struct Ability {
    countdown: u32,
}

impl Ability {
    pub fn gen_particle(pos: Vec2, v: Option<Vec2>, level: Option<u8>) -> Particle {
        let mut particle = Particle::new(Box::new(Ability { countdown: 0 }), pos, v, level);
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
    fn current_countdown(&self) -> u32 {
        self.countdown
    }
    fn max_countdown(&self, level: u8) -> u32 {
        2 ^ (level as u32) * 50
    }
    fn reset_countdown(&mut self, level: u8) {
        self.countdown = self.max_countdown(level);
    }
    fn tick_countdown(&mut self) -> u32 {
        if self.countdown - 1 <= 0 {
            self.countdown = 0;
        } else {
            self.countdown = self.countdown - 1;
        }
        self.countdown
    }
}

pub fn build_particle_tmm(
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> TMM {
    let texture = asset_server.load("images/icons/copy-fill.png");
    let mesh = meshes.add((shape::Box::new(RADIUS * 1.6, RADIUS * 1.6, 0.0)).into());
    let material = materials.add(COLOR.into());
    TMM {
        texture,
        mesh,
        material,
    }
}

pub fn build_particle_sprite(
    commands: &mut Commands,
    _particle_tmm: &Res<ParticleTMM>,
    bundle: impl Bundle,
    pos: Option<Vec2>,
    v: Option<Vec2>,
    level: Option<u8>,
) {
    let pos = match pos {
        Some(pos) => pos,
        None => field::gen_random_pos_in_field(RADIUS * 2.0),
    };
    let particle = Particle::create(ParticleType::Trigger, pos, v, level);
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
    update_particle_sprite(commands, root_entity, side_ratio);
}

pub fn update_particle_sprite(commands: &mut Commands, root_entity: Entity, side_ratio: f32) {
    commands.entity(root_entity).despawn_descendants();
    commands.entity(root_entity).with_children(|parent| {
        parent.spawn(ShapeBundle::ngon(
            &ShapeConfig {
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                color: COLOR,
                ..ShapeConfig::default_2d()
            },
            3.0,
            RADIUS,
        ));
        parent.spawn(ShapeBundle::arc(
            &ShapeConfig {
                transform: Transform::from_xyz(0.0, 0.0, 3.0),
                color: COLOR,
                hollow: true,
                thickness: SIDE_THICKNESS,
                ..ShapeConfig::default_2d()
            },
            RADIUS * 1.5,
            0.0,
            TAU * side_ratio,
        ));
        parent.spawn(ShapeBundle::circle(
            &ShapeConfig {
                transform: Transform::from_xyz(0.0, RADIUS * 1.5, 3.0),
                color: COLOR,
                hollow: false,
                ..ShapeConfig::default_2d()
            },
            alpha::RADIUS,
        ));
    });
}

const LEVEL_INIT_BIAS_COUNT: u32 = 10;

pub fn update_particle_level(particle: &mut Particle, total_alpha_count: u32) {
    let mut level = 0;
    if total_alpha_count < LEVEL_INIT_BIAS_COUNT + 2 ^ (MIN_LEVEL + 1) as u32 {
        level = MIN_LEVEL;
    }
    if level == 0 {
        for i in MIN_LEVEL..=MAX_LEVEL {
            if total_alpha_count >= LEVEL_INIT_BIAS_COUNT + 2 ^ (i + 1) as u32
                && total_alpha_count < LEVEL_INIT_BIAS_COUNT + 2 ^ (i + 2) as u32
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