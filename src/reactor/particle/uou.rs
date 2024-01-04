use crate::reactor::{field, particle::*};
use bevy::sprite::MaterialMesh2dBundle;

pub const RADIUS: f32 = 8.0;
pub const COLOR: Color = Color::rgb(1.0, 0.84, 0.2);
const MIN_LEVEL: u8 = 1;
const MAX_LEVEL: u8 = 5;
const MIN_V: f32 = 0.0;
const MAX_V: f32 = 0.0;

pub struct Ability;

impl Ability {
    pub fn gen_particle(
        pos: Vec2,
        direction: Option<Vec2>,
        level: Option<u8>,
        canvas_entity: Option<Entity>,
    ) -> Particle {
        Particle::new(Box::new(Ability), pos, direction, level, canvas_entity)
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
}

pub fn build_particle_sprite(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    bundle: impl Bundle,
    pos: Option<Vec2>,
    direction: Option<Vec2>,
    level: Option<u8>,
) {
    let pos = match pos {
        Some(pos) => pos,
        None => field::gen_random_pos_in_field(RADIUS * 2.0),
    };
    let particle = Particle::create(ParticleType::Uou, pos, direction, level, None);
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
                mesh: meshes.add(shape::Circle::new(RADIUS).into()).into(),
                material: materials.add(ColorMaterial::from(COLOR)),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            });
        });
}
