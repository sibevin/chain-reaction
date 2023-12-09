use crate::reactor::particle::*;
use bevy::prelude::*;

pub fn is_hit(p1: &Particle, p2: &Particle) -> bool {
    p1.pos().distance(p2.pos()) <= p1.radius() + p2.radius()
}

pub fn through_hit_v_change(p1: &Particle, p2: &Particle) -> (PosV, PosV) {
    if is_hit(p1, p2) {
        let new_v1 = p1.v().normalize() * p2.v().length() / (p1.v().length() + p2.v().length());
        let new_v2 = p2.v().normalize() * p1.v().length() / (p1.v().length() + p2.v().length());
        return (
            PosV {
                pos: p1.pos(),
                v: new_v1,
            },
            PosV {
                pos: p2.pos(),
                v: new_v2,
            },
        );
    }
    (p1.pos_v(), p2.pos_v())
}

pub fn reflective_hit_v_change(p1: &Particle, p2: &Particle) -> (PosV, PosV) {
    if is_hit(p1, p2) {
        let vt_1: Vec2 = p1.pos() - p2.pos();
        let vp_1: Vec2 = p1.v().project_onto(vt_1);
        let new_v1 = -vp_1 + (p1.v() - vp_1);

        let vt_2: Vec2 = p2.pos() - p1.pos();
        let vp_2: Vec2 = p2.v().project_onto(vt_2);
        let new_v2 = -vp_2 + (p2.v() - vp_2);

        return (
            PosV {
                pos: p1.pos() + new_v1,
                v: new_v1,
            },
            PosV {
                pos: p2.pos() + new_v2,
                v: new_v2,
            },
        );
    }
    (p1.pos_v(), p2.pos_v())
}
