use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_tweening::*;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;
use std::time::Duration;

mod kind;
mod lens;
mod plugin;
mod status;

pub use plugin::AnimeEffectPlugin;
pub use status::{despawn_anime_effect, AnimeEffectStatus};

pub struct AnimeEffectParam {
    pub kind: AnimeEffectKind,
    pub color: Color,
    pub pos_1: Vec2,
    pub pos_2: Vec2,
    pub width_start: f32,
    pub width_end: f32,
}

#[derive(Clone, PartialEq)]
pub enum AnimeEffectKind {
    LineQ,
    CircleQ,
    LineShoot,
    CircleShoot,
    RectShoot,
}

#[derive(Component)]
pub struct AnimeEffect {
    pub kind: AnimeEffectKind,
    pub segments: Vec<[Vec2; 4]>,
    pub pos_1: Vec2,
    pub pos_2: Vec2,
    pub layer: u64,
    pub delta: f32,
    pub color: Color,
    pub width: f32,
    pub radius: f32,
    pub root_entity: Entity,
    pub is_done: bool,
}

pub const ANIME_EFFECT_DONE_EVENT: u64 = 4;

pub fn insert_anime_effect(commands: &mut Commands, param: AnimeEffectParam, bundle: impl Bundle) {
    let entity = kind::fetch_builder(param.kind.clone()).create(commands, param);
    commands.entity(entity).insert(bundle);
}

pub fn update_anime_effect(commands: &mut Commands, ae: &mut AnimeEffect) {
    kind::fetch_builder(ae.kind.clone()).draw(commands, ae);
}

pub fn clear_anime_effect(mut commands: Commands, ae_query: Query<Entity, With<AnimeEffect>>) {
    for ae_entity in ae_query.iter() {
        if let Some(entity_commands) = commands.get_entity(ae_entity) {
            entity_commands.despawn_recursive()
        }
    }
}
