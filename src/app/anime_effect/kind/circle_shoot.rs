use super::*;
use crate::app::theme::*;

pub struct AnimeEffectKindCircleShoot;

impl AnimeEffectKindBase for AnimeEffectKindCircleShoot {
    fn create(&self, commands: &mut Commands, param: AnimeEffectParam) -> Entity {
        let root_entity = commands
            .spawn((SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, ANIME_EFFECT_Z_INDEX),
                ..default()
            },))
            .id();
        let ae = AnimeEffect {
            kind: param.kind,
            segments: vec![],
            pos_1: param.pos_1,
            pos_2: param.pos_2,
            layer: 0,
            delta: 0.0,
            color: param.color,
            width: param.width_start,
            radius: (param.pos_2 - param.pos_1).length() / 2.0 + param.width_start * 2.0,
            root_entity,
            is_done: false,
        };
        let tween = Tween::new(
            EaseFunction::CubicOut,
            Duration::from_millis(500),
            lens::AnimeEffectLens::new((param.width_start, param.width_end)),
        );
        commands
            .entity(root_entity)
            .insert((ae, Animator::new(tween)));
        root_entity
    }

    fn draw(&self, commands: &mut Commands, ae: &mut AnimeEffect) {
        if ae.is_done {
            return;
        }
        if let Some(mut entity_commands) = commands.get_entity(ae.root_entity) {
            entity_commands.despawn_descendants();
            entity_commands.with_children(|parent| {
                parent
                    .spawn((SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 0.0001),
                        ..default()
                    },))
                    .with_children(|parent| {
                        let circle = shapes::Circle {
                            radius: ae.radius,
                            center: ae.pos_1,
                        };
                        let geo_builder = GeometryBuilder::new().add(&circle);
                        parent.spawn((
                            ShapeBundle {
                                path: geo_builder.build(),
                                ..default()
                            },
                            Stroke::new(ae.color, ae.width),
                        ));
                    });
                let start_angle = -PI / 2.0;
                let angle = PI * 2.0 * ae.delta;
                let mut path_builder = PathBuilder::new();
                path_builder.move_to(Vec2::ZERO);
                path_builder.line_to(Vec2::from_angle(angle * 2.0) * ae.radius * 1.2);
                path_builder.arc(
                    Vec2::ZERO,
                    Vec2::new(ae.radius * 1.2, ae.radius * 1.2),
                    PI * 2.0 - angle,
                    0.0,
                );
                path_builder.close();
                parent.spawn((
                    ShapeBundle {
                        path: path_builder.build(),
                        spatial: SpatialBundle {
                            transform: Transform::from_xyz(ae.pos_1.x, ae.pos_1.y, 0.0002)
                                .with_rotation(Quat::from_rotation_z(start_angle - angle * 2.0)),
                            ..default()
                        },
                        ..default()
                    },
                    Fill::color(BG_COLOR),
                ));
                let dot_pos = Vec2::from_angle(start_angle - angle) * ae.radius;
                let circle = shapes::Circle {
                    radius: ae.width * 1.5,
                    center: dot_pos,
                };
                let geo_builder = GeometryBuilder::new().add(&circle);
                parent.spawn((
                    ShapeBundle {
                        path: geo_builder.build(),
                        spatial: SpatialBundle {
                            transform: Transform::from_xyz(ae.pos_1.x, ae.pos_1.y, 0.0003),
                            ..default()
                        },
                        ..default()
                    },
                    Fill::color(ae.color),
                ));
            });
            if ae.delta == 1.0 {
                ae.is_done = true;
            }
        };
    }
}
