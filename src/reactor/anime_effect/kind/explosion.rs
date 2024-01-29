use crate::reactor::anime_effect::kind::*;

pub struct AnimeEffectKindExplosion;

const EXPLOSION_BORDER: f32 = 3.0;

impl AnimeEffectKindBase for AnimeEffectKindExplosion {
    fn create(&self, commands: &mut Commands, param: AnimeEffectParam) {
        let color = match param.shape {
            AnimeEffectShape::Circle => reactor::particle::alpha::COLOR,
            AnimeEffectShape::Square => reactor::particle::control::COLOR,
            AnimeEffectShape::Hexagon => reactor::particle::hyper::COLOR,
            AnimeEffectShape::Triangle => reactor::particle::trigger::COLOR,
        };
        let root_entity = commands
            .spawn((SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
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
            kind: param.kind,
            shape: param.shape,
            start_pos: param.start_pos,
            current_pos: param.start_pos,
            color,
            radius: 0.0,
            rotation: init_rotation,
            rotation_delta,
            border: EXPLOSION_BORDER,
            root_entity,
        };
        let tween = Tween::new(
            EaseFunction::CubicIn,
            Duration::from_millis(2500),
            lens::AnimeEffectLens::new(
                0.0,
                0.1,
                EXPLOSION_BORDER * 2.0,
                param.start_pos,
                WINDOW_W,
                0.0,
                EXPLOSION_BORDER,
                param.end_pos,
            ),
        )
        .with_completed_event(ANIME_EFFECT_DONE_EVENT);
        commands
            .entity(root_entity)
            .insert((ae, Animator::new(tween)));
    }

    fn draw(&self, commands: &mut Commands, ae: &AnimeEffect) {
        if let Some(mut entity_commands) = commands.get_entity(ae.root_entity) {
            entity_commands.despawn_descendants();
            entity_commands.with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        transform: Transform::from_rotation(Quat::from_rotation_z(ae.rotation))
                            .with_translation(Vec3::new(ae.current_pos.x, ae.current_pos.y, -10.0)),
                        sprite: Sprite {
                            color: ae.color,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        match ae.shape {
                            AnimeEffectShape::Circle => {
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
                            AnimeEffectShape::Square => {
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
                            AnimeEffectShape::Hexagon => {
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
                            AnimeEffectShape::Triangle => {
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
}
