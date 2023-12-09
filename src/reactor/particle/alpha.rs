use crate::reactor::{field, particle::*, tmm::*};
use bevy::sprite::MaterialMesh2dBundle;

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

pub fn build_particle_tmm(
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> TMM {
    let texture = asset_server.load("images/icons/copy-fill.png");
    let mesh = meshes.add((shape::Circle::new(RADIUS)).into());
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
    let particle = Particle::create(ParticleType::Alpha, pos, v, level);
    let tmm = particle_tmm.get(ParticleType::Alpha).as_ref().unwrap();
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
                global_transform: GlobalTransform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            });
        });
}
