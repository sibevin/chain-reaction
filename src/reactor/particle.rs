use crate::reactor;
use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::fmt;

pub mod alpha;
pub mod control;
pub mod hyper;
pub mod trigger;
pub mod uou;

const MOVING_TAILING_COUNT: usize = 3;
const MIN_V_LENGTH: f32 = 1.0;
const MAX_V_LENGTH: f32 = 4.0;
const SIDE_THICKNESS: f32 = 0.3;

#[derive(Component, Debug, PartialEq)]
pub enum ParticleType {
    Alpha,
    Hyper,
    Control,
    Trigger,
    Uou,
}

pub struct PosV {
    pub pos: Vec2,
    pub v: Vec2,
}

#[derive(Component)]
pub struct Particle {
    ability: Box<dyn ParticleAbility + Send + Sync>,
    level: u8,
    pos: Vec2,
    v: Vec2,
    is_moving: bool,
    tailings: [Vec2; MOVING_TAILING_COUNT],
}

pub trait ParticleAbility {
    fn particle_type(&self) -> ParticleType;
    fn min_level(&self) -> u8;
    fn max_level(&self) -> u8;
    fn radius(&self) -> f32;
    fn color(&self) -> Color;
    fn current_countdown(&self) -> u32 {
        0
    }
    fn max_countdown(&self, _level: u8) -> u32 {
        1
    }
    fn reset_countdown(&mut self, _level: u8) {}
    fn tick_countdown(&mut self) -> u32 {
        0
    }
}

impl Particle {
    pub fn create(
        particle_type: ParticleType,
        pos: Vec2,
        v: Option<Vec2>,
        level: Option<u8>,
    ) -> Self {
        match particle_type {
            ParticleType::Alpha => alpha::Ability::gen_particle(pos, v, level),
            ParticleType::Hyper => hyper::Ability::gen_particle(pos, v, level),
            ParticleType::Control => control::Ability::gen_particle(pos, v, level),
            ParticleType::Trigger => trigger::Ability::gen_particle(pos, v, level),
            ParticleType::Uou => uou::Ability::gen_particle(pos, v, level),
        }
    }
    pub fn new(
        ability: Box<dyn ParticleAbility + Send + Sync>,
        pos: Vec2,
        v: Option<Vec2>,
        level: Option<u8>,
    ) -> Self {
        let v = match v {
            Some(v) => v,
            None => Particle::gen_random_v(None),
        };
        let level = match level {
            Some(level) => level,
            None => ability.min_level(),
        };
        Self {
            ability,
            level,
            pos,
            v,
            is_moving: true,
            tailings: [pos; MOVING_TAILING_COUNT],
        }
    }
    pub fn particle_type(&self) -> ParticleType {
        self.ability.particle_type()
    }
    pub fn radius(&self) -> f32 {
        self.ability.radius()
    }
    pub fn color(&self) -> Color {
        self.ability.color()
    }
    pub fn pos(&self) -> Vec2 {
        self.pos
    }
    pub fn v(&self) -> Vec2 {
        self.v
    }
    pub fn pos_v(&self) -> PosV {
        PosV {
            pos: self.pos,
            v: self.v,
        }
    }
    pub fn level(&self) -> u8 {
        self.level
    }
    pub fn tailings(&self) -> [Vec2; MOVING_TAILING_COUNT] {
        self.tailings
    }
    pub fn toggle_moving(&mut self) {
        self.is_moving = !self.is_moving;
    }
    pub fn travel(&mut self) -> Vec2 {
        let ori_pos = self.pos;
        self.pos = Particle::next_pos(ori_pos, self.v, self.radius());
        self.v = Particle::next_v(ori_pos, self.v, self.radius());
        self.pos
    }
    pub fn jump(&mut self, pos: Vec2) {
        self.pos = pos;
    }
    pub fn update_level(&mut self, delta: i32) {
        let new_level = (self.level as i32 + delta).clamp(
            self.ability.min_level() as i32,
            self.ability.max_level() as i32,
        );
        self.level = new_level as u8;
    }
    pub fn level_ratio(&self) -> f32 {
        self.level as f32 / self.ability.max_level() as f32
    }
    pub fn countdown_ratio(&self) -> f32 {
        self.ability.current_countdown() as f32 / self.ability.max_countdown(self.level) as f32
    }
    pub fn max_countdown(&self) -> u32 {
        self.ability.max_countdown(self.level)
    }
    pub fn reset_countdown(&mut self) {
        self.ability.reset_countdown(self.level)
    }
    pub fn tick_countdown(&mut self) -> u32 {
        self.ability.tick_countdown()
    }
    fn next_pos(pos: Vec2, v: Vec2, r: f32) -> Vec2 {
        let field_rect = reactor::field::get_field_rect(0.0);
        let mut new_pos = pos + v;
        if new_pos.x + r > field_rect.max.x && v.x > 0.0 {
            new_pos.x = field_rect.max.x * 2.0 - new_pos.x - r * 2.0;
        } else if new_pos.x - r < field_rect.min.x && v.x < 0.0 {
            new_pos.x = field_rect.min.x * 2.0 - new_pos.x + r * 2.0;
        }
        if new_pos.y + r > field_rect.max.y && v.y > 0.0 {
            new_pos.y = field_rect.max.y * 2.0 - new_pos.y - r * 2.0;
        } else if new_pos.y - r < field_rect.min.y && v.y < 0.0 {
            new_pos.y = field_rect.min.y * 2.0 - new_pos.y + r * 2.0;
        }
        new_pos
    }
    fn next_v(pos: Vec2, v: Vec2, r: f32) -> Vec2 {
        let field_rect = reactor::field::get_field_rect(0.0);
        let mut new_v = v;
        let new_pos = pos + v;
        if new_pos.x + r > field_rect.max.x && v.x > 0.0 {
            new_v.x = -new_v.x;
        } else if new_pos.x - r < field_rect.min.x && v.x < 0.0 {
            new_v.x = -new_v.x;
        }
        if new_pos.y + r > field_rect.max.y && v.y > 0.0 {
            new_v.y = -new_v.y;
        } else if new_pos.y - r < field_rect.min.y && v.y < 0.0 {
            new_v.y = -new_v.y;
        }
        new_v
    }
    pub fn gen_random_v(direction: Option<Vec2>) -> Vec2 {
        let mut rng = thread_rng();
        let v = direction
            .unwrap_or({
                let angle = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
                Vec2::new(angle.cos(), angle.sin())
            })
            .normalize();
        v * rng.gen_range(MIN_V_LENGTH..MAX_V_LENGTH)
    }
}

impl fmt::Debug for Particle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Particle")
            .field("particle_type", &self.particle_type())
            .field("level", &self.level)
            .field("pos", &self.pos)
            .field("v", &self.v)
            .field("is_moving", &self.is_moving)
            .field("color", &self.color())
            .finish()
    }
}
