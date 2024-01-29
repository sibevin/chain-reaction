use crate::reactor;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_tweening::*;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;
use std::time::Duration;

use crate::app::WINDOW_W;

use self::kind::AnimeEffectKindBase;

mod kind;
mod lens;

pub struct AnimeEffectParam {
    pub kind: AnimeEffectKind,
    pub shape: AnimeEffectShape,
    pub start_pos: Vec2,
    pub end_pos: Vec2,
}

#[derive(Clone, PartialEq)]
pub enum AnimeEffectKind {
    Explosion,
    Bullet,
}

#[derive(PartialEq)]
pub enum AnimeEffectShape {
    Circle,
    Square,
    Hexagon,
    Triangle,
}

#[derive(Component)]
pub struct AnimeEffect {
    pub kind: AnimeEffectKind,
    pub shape: AnimeEffectShape,
    pub start_pos: Vec2,
    pub current_pos: Vec2,
    pub radius: f32,
    pub color: Color,
    pub rotation: f32,
    pub rotation_delta: f32,
    pub border: f32,
    pub root_entity: Entity,
}

impl AnimeEffect {
    fn kind_builder(&self) -> &dyn AnimeEffectKindBase {
        kind::fetch_builder(self.kind.clone())
    }
}

pub const ANIME_EFFECT_DONE_EVENT: u64 = 4;
const AE_ROTATION_DELTA: f32 = PI / 40.0;

pub fn insert_anime_effect(commands: &mut Commands, param: AnimeEffectParam) {
    kind::fetch_builder(param.kind.clone()).create(commands, param);
}

pub fn update_anime_effect(commands: &mut Commands, ae: &AnimeEffect) {
    ae.kind_builder().draw(commands, ae);
}

pub fn clear_anime_effect(mut commands: Commands, ae_query: Query<Entity, With<AnimeEffect>>) {
    for ae_entity in ae_query.iter() {
        if let Some(entity_commands) = commands.get_entity(ae_entity) {
            entity_commands.despawn_recursive()
        }
    }
}
