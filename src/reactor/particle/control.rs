use crate::{
    app,
    reactor::{field, particle::*, tmm::*},
};
use bevy::sprite::MaterialMesh2dBundle;
use bevy_vector_shapes::prelude::*;
use std::f32::consts::{PI, TAU};

const MIN_LEVEL: u8 = 1;
const MAX_LEVEL: u8 = 8;
const RADIUS: f32 = 12.0;
const COLOR: Color = Color::LIME_GREEN;

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
    fn current_countdown(&self) -> u32 {
        self.countdown
    }
    fn max_countdown(&self, level: u8) -> u32 {
        2 ^ (level as u32)
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
    let mesh = meshes.add((shape::Box::new(RADIUS * 1.8, RADIUS * 1.8, 0.0)).into());
    let material = materials.add(COLOR.into());
    TMM {
        texture,
        mesh,
        material,
    }
}

pub fn build_particle_sprite(
    commands: &mut Commands,
    particle_tmm: &Res<ParticleTMM>,
    bundle: impl Bundle,
    pos: Option<Vec2>,
    v: Option<Vec2>,
    level: Option<u8>,
) {
    let pos = match pos {
        Some(pos) => pos,
        None => field::gen_random_pos_in_field(RADIUS * 2.0),
    };
    let particle = Particle::create(ParticleType::Control, pos, v, level);
    let tmm = particle_tmm.get(ParticleType::Control).as_ref().unwrap();
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
            parent.spawn(MaterialMesh2dBundle {
                mesh: tmm.mesh.clone().into(),
                material: tmm.material.clone(),
                global_transform: GlobalTransform::from_xyz(0.0, 0.0, 1.0),
                ..Default::default()
            });
            parent.spawn(ShapeBundle::ngon(
                &ShapeConfig {
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    color: app::ui::BG_COLOR,
                    ..ShapeConfig::default_2d()
                },
                MAX_LEVEL as f32,
                RADIUS * 0.7,
            ));
            let start_angle = 0.0; //-PI / MAX_LEVEL as f32;
            parent.spawn(ShapeBundle::arc(
                &ShapeConfig {
                    transform: Transform::from_xyz(0.0, 0.0, 3.0),
                    hollow: false,
                    cap: Cap::None,
                    color: COLOR,
                    ..ShapeConfig::default_2d()
                },
                RADIUS * 0.8,
                start_angle,
                start_angle + TAU * (5. / MAX_LEVEL as f32),
            ));
            for i in 1..MAX_LEVEL {
                let angle = PI * 2.0 * i as f32 / MAX_LEVEL as f32;
                parent.spawn(ShapeBundle::line(
                    &ShapeConfig {
                        transform: Transform::from_xyz(0.0, 0.0, 4.0),
                        color: COLOR,
                        cap: Cap::None,
                        thickness: SIDE_THICKNESS * 3.0,
                        ..ShapeConfig::default_2d()
                    },
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(angle.cos(), angle.sin(), 0.0) * RADIUS * 0.8,
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
                TAU * (2. / 3.0),
            ));
        });
}
