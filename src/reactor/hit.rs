use crate::reactor::particle::*;
use bevy::prelude::*;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::particle;

pub fn is_hit(p1: &Particle, p2: &Particle) -> bool {
    p1.pos().distance(p2.pos()) <= p1.radius + p2.radius
}

#[derive(Debug, Copy, Clone)]
pub enum HitAction {
    MoveOnly,
    AlphaHit(u32),
    UouHit,
    Kill,
    Release(u32),
}

type EntityParticle<'a> = (Rc<RefCell<Entity>>, Rc<RefCell<Mut<'a, Particle>>>);

pub fn detect_hit(
    particle_query: &mut Query<(Entity, &mut Particle, &mut Transform), With<Particle>>,
) -> HashMap<Entity, HitAction> {
    let mut particles: Vec<EntityParticle> = Vec::new();
    for (e, p, _) in particle_query.iter_mut() {
        particles.push((Rc::new(RefCell::new(e)), Rc::new(RefCell::new(p))));
    }
    let mut hit_map: HashMap<Entity, HitAction> = HashMap::new();
    for (i, (e1, p1)) in particles.iter().enumerate() {
        if p1.borrow().state != particle::ParticleState::Running {
            continue;
        }
        for (j, (e2, p2)) in particles.iter().enumerate() {
            if p2.borrow().state != particle::ParticleState::Running {
                continue;
            }
            if j > i && is_hit(p1.borrow().as_ref(), p2.borrow().as_ref()) {
                record_hit_action(
                    &mut hit_map,
                    *e1.borrow(),
                    p1.borrow().as_ref(),
                    *e2.borrow(),
                    p2.borrow().as_ref(),
                )
            }
        }
    }
    hit_map
}

pub fn record_hit_action(
    hit_map: &mut HashMap<Entity, HitAction>,
    e1: Entity,
    p1: &Particle,
    e2: Entity,
    p2: &Particle,
) {
    let mut e1_action: HitAction = hit_map.get(&e1).copied().unwrap_or(HitAction::MoveOnly);
    let mut e2_action: HitAction = hit_map.get(&e2).copied().unwrap_or(HitAction::MoveOnly);
    match p1.particle_type() {
        ParticleType::Alpha => match p2.particle_type() {
            ParticleType::Alpha => {
                if p1.countdown_ratio() == 0.0 && p1.countdown_ratio() == 0.0 {
                    if p1.level() == 1 && p2.level() == 1 {
                        e1_action = HitAction::Release(2_u32.pow(p1.level() as u32));
                        e2_action = HitAction::MoveOnly;
                    } else if p1.level() == 1 && p2.level() != 1 {
                        e2_action = HitAction::Release(2_u32.pow(p2.level() as u32));
                    } else if p1.level() != 1 && p2.level() == 1 {
                        e1_action = HitAction::Release(2_u32.pow(p1.level() as u32));
                    } else {
                        // p1.level and p2.level both > 1
                        e1_action = HitAction::Release(2_u32.pow(p1.level() as u32));
                        e2_action = HitAction::Release(2_u32.pow(p2.level() as u32));
                    }
                }
            }
            ParticleType::Control => {
                e1_action = HitAction::Kill;
                match e2_action {
                    HitAction::MoveOnly => {
                        e2_action = HitAction::AlphaHit(1);
                    }
                    HitAction::AlphaHit(count) => {
                        e2_action = HitAction::AlphaHit(count + 1);
                    }
                    _ => (),
                }
            }
            ParticleType::Uou => {
                e2_action = HitAction::Kill;
            }
            _ => (),
        },
        ParticleType::Control => match p2.particle_type() {
            ParticleType::Alpha => {
                match e1_action {
                    HitAction::MoveOnly => {
                        e1_action = HitAction::AlphaHit(1);
                    }
                    HitAction::AlphaHit(count) => {
                        e1_action = HitAction::AlphaHit(count + 1);
                    }
                    _ => (),
                }
                e2_action = HitAction::Kill;
            }
            ParticleType::Uou => {
                e1_action = HitAction::UouHit;
            }
            ParticleType::Control => {
                e1_action = HitAction::Kill;
                e2_action = HitAction::Kill;
            }
            ParticleType::Hyper => {
                e1_action = HitAction::Kill;
            }
            _ => (),
        },
        ParticleType::Uou => match p2.particle_type() {
            ParticleType::Alpha => {
                e1_action = HitAction::Kill;
            }
            ParticleType::Control => {
                e2_action = HitAction::UouHit;
            }
            ParticleType::Hyper => {
                e2_action = HitAction::UouHit;
            }
            ParticleType::Trigger => {
                e2_action = HitAction::UouHit;
            }
            _ => (),
        },
        ParticleType::Hyper => match p2.particle_type() {
            ParticleType::Uou => {
                e1_action = HitAction::UouHit;
            }
            ParticleType::Control => {
                e2_action = HitAction::Kill;
            }
            _ => (),
        },
        ParticleType::Trigger => {
            if p2.particle_type() == ParticleType::Uou {
                e1_action = HitAction::UouHit;
            }
        }
    }
    hit_map.insert(e1, e1_action);
    hit_map.insert(e2, e2_action);
}
