use crate::reactor::anime_effect::kind::*;

pub struct AnimeEffectKindBullet;

const BULLET_RADIUS: f32 = 6.0;

impl AnimeEffectKindBase for AnimeEffectKindBullet {
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
        let ae = AnimeEffect {
            kind: param.kind,
            shape: param.shape,
            start_pos: param.start_pos,
            current_pos: param.start_pos,
            color,
            radius: BULLET_RADIUS,
            rotation: 0.0,
            rotation_delta: 0.0,
            border: 0.0,
            root_entity,
        };
        let tween = Tween::new(
            EaseFunction::CubicIn,
            Duration::from_millis(500),
            lens::AnimeEffectLens::new(
                (BULLET_RADIUS, BULLET_RADIUS),
                (0.3, 0.3),
                (0.0, 0.0),
                (param.start_pos, param.end_pos),
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
                let mut path_builder = PathBuilder::new();
                path_builder.move_to(ae.start_pos);
                path_builder.line_to(ae.current_pos);
                parent.spawn((
                    ShapeBundle {
                        path: path_builder.build(),
                        ..default()
                    },
                    Stroke::new(ae.color, BULLET_RADIUS),
                ));
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
                                    Fill::color(ae.color),
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
                                    Fill::color(ae.color),
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
                                    Fill::color(ae.color),
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
                                    Fill::color(ae.color),
                                ));
                            }
                        };
                    });
            });
        }
    }
}
