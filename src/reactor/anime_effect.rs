use crate::reactor;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_tweening::*;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;
use std::time::Duration;

use crate::app::WINDOW_W;

#[derive(Component, Debug, PartialEq)]
pub enum AnimeEffectType {
    Circle,
    Square,
    Hexagon,
    Triangle,
}

#[derive(Component)]
pub struct AnimeEffect {
    pub ae_type: AnimeEffectType,
    pub radius: f32,
    pub color: Color,
    pub rotation: f32,
    pub rotation_delta: f32,
    pub border: f32,
    pub root_entity: Entity,
}

struct AnimeEffectLens {
    start_radius: f32,
    start_color_alpha: f32,
    start_border: f32,
    end_radius: f32,
    end_color_alpha: f32,
    end_border: f32,
}

impl Lens<AnimeEffect> for AnimeEffectLens {
    fn lerp(&mut self, target: &mut AnimeEffect, ratio: f32) {
        target.radius = self.start_radius + (self.end_radius - self.start_radius) * ratio;
        let color_alpha =
            self.start_color_alpha + (self.end_color_alpha - self.start_color_alpha) * ratio;
        target.color.set_a(color_alpha);
        target.border = self.start_border + (self.end_border - self.start_border) * ratio;
        target.rotation += target.rotation_delta;
    }
}

pub const ANIME_EFFECT_DONE_EVENT: u64 = 4;
const AE_BORDER: f32 = 3.0;
const AE_ROTATION_DELTA: f32 = PI / 40.0;

pub fn insert_anime_effect(commands: &mut Commands, ae_type: AnimeEffectType, pos: Vec2) {
    let color = match ae_type {
        AnimeEffectType::Circle => reactor::particle::alpha::COLOR,
        AnimeEffectType::Square => reactor::particle::control::COLOR,
        AnimeEffectType::Hexagon => reactor::particle::hyper::COLOR,
        AnimeEffectType::Triangle => reactor::particle::trigger::COLOR,
    };
    let root_entity = commands
        .spawn((SpriteBundle {
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            sprite: Sprite { color, ..default() },
            ..default()
        },))
        .id();
    let mut rng = thread_rng();
    let init_rotation = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
    let rotation_delta = if rng.gen_range(-1.0..1.0) < 0.0 {
        AE_ROTATION_DELTA * -1.0
    } else {
        AE_ROTATION_DELTA
    };
    let ae = AnimeEffect {
        ae_type,
        color,
        radius: 0.0,
        rotation: init_rotation,
        rotation_delta,
        border: AE_BORDER,
        root_entity,
    };
    let tween = Tween::new(
        EaseFunction::CubicIn,
        Duration::from_millis(2500),
        AnimeEffectLens {
            start_radius: 0.0,
            start_color_alpha: 0.1,
            start_border: AE_BORDER * 2.0,
            end_radius: WINDOW_W,
            end_color_alpha: 0.0,
            end_border: AE_BORDER,
        },
    )
    .with_completed_event(ANIME_EFFECT_DONE_EVENT);
    commands
        .entity(root_entity)
        .insert((ae, Animator::new(tween)));
}

pub fn update_anime_effect(commands: &mut Commands, ae: &AnimeEffect) {
    if let Some(mut entity_commands) = commands.get_entity(ae.root_entity) {
        entity_commands.despawn_descendants();
        entity_commands.with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    transform: Transform::from_rotation(Quat::from_rotation_z(ae.rotation))
                        .with_translation(Vec3::new(0.0, 0.0, -10.0)),
                    sprite: Sprite {
                        color: ae.color,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    match ae.ae_type {
                        AnimeEffectType::Circle => {
                            let shape = shapes::Circle {
                                radius: ae.radius,
                                center: Vec2::new(0.0, 0.0),
                            };
                            parent.spawn((
                                ShapeBundle {
                                    path: GeometryBuilder::build_as(&shape),
                                    ..default()
                                },
                                Stroke::new(ae.color, ae.border),
                            ));
                        }
                        AnimeEffectType::Square => {
                            let shape = shapes::RegularPolygon {
                                sides: 4,
                                feature: shapes::RegularPolygonFeature::Radius(
                                    ae.radius * 2.0_f32.sqrt(),
                                ),
                                ..shapes::RegularPolygon::default()
                            };
                            parent.spawn((
                                ShapeBundle {
                                    path: GeometryBuilder::build_as(&shape),
                                    ..default()
                                },
                                Stroke::new(ae.color, ae.border),
                            ));
                        }
                        AnimeEffectType::Hexagon => {
                            let shape = shapes::RegularPolygon {
                                sides: 6,
                                feature: shapes::RegularPolygonFeature::Radius(ae.radius),
                                ..shapes::RegularPolygon::default()
                            };
                            parent.spawn((
                                ShapeBundle {
                                    path: GeometryBuilder::build_as(&shape),
                                    ..default()
                                },
                                Stroke::new(ae.color, ae.border),
                            ));
                        }
                        AnimeEffectType::Triangle => {
                            let shape = shapes::RegularPolygon {
                                sides: 3,
                                feature: shapes::RegularPolygonFeature::Radius(ae.radius),
                                ..shapes::RegularPolygon::default()
                            };
                            parent.spawn((
                                ShapeBundle {
                                    path: GeometryBuilder::build_as(&shape),
                                    ..default()
                                },
                                Stroke::new(ae.color, ae.border),
                            ));
                        }
                    };
                });
        });
    }
}

pub fn clear_anime_effect(mut commands: Commands, ae_query: Query<Entity, With<AnimeEffect>>) {
    for ae_entity in ae_query.iter() {
        if let Some(entity_commands) = commands.get_entity(ae_entity) {
            entity_commands.despawn_recursive()
        }
    }
}
