use super::*;
use crate::app::theme::*;

pub struct AnimeEffectKindRectShoot;

const COVER_LINE_W_RATIO: f32 = 3.0;

impl AnimeEffectKindBase for AnimeEffectKindRectShoot {
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
            radius: 0.0,
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
                let center_pos = (ae.pos_1 + ae.pos_2) / 2.0;
                let w = ae.pos_2.x - ae.pos_1.x;
                let h = ae.pos_1.y - ae.pos_2.y;
                parent
                    .spawn((SpatialBundle {
                        transform: Transform::from_xyz(center_pos.x, center_pos.y, 0.0001),
                        ..default()
                    },))
                    .with_children(|parent| {
                        let rect = shapes::Rectangle {
                            extents: Vec2::new(w, h),
                            ..default()
                        };
                        let geo_builder = GeometryBuilder::new().add(&rect);
                        parent.spawn((
                            ShapeBundle {
                                path: geo_builder.build(),
                                ..default()
                            },
                            Stroke::new(ae.color, ae.width),
                        ));
                    });
                parent
                    .spawn((SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 0.0002),
                        ..default()
                    },))
                    .with_children(|parent| {
                        let circle = shapes::Circle {
                            radius: ae.width * 1.5,
                            center: ae.pos_2,
                        };
                        let geo_builder = GeometryBuilder::new().add(&circle);
                        parent.spawn((
                            ShapeBundle {
                                path: geo_builder.build(),
                                ..default()
                            },
                            Fill::color(ae.color),
                        ));
                    });
                if ae.delta < 1.0 {
                    parent
                        .spawn((SpatialBundle {
                            transform: Transform::from_xyz(0.0, 0.0, 0.0003),
                            ..default()
                        },))
                        .with_children(|parent| {
                            let mut path_builder = PathBuilder::new();
                            let cover_line_w = ae.width * COVER_LINE_W_RATIO;
                            let anchor_pos = Vec2::new(
                                ae.pos_1.x + ae.delta * (w + h + cover_line_w * 2.0),
                                ae.pos_1.y,
                            );
                            path_builder.move_to(anchor_pos);
                            path_builder.line_to(Vec2::new(anchor_pos.x - h, ae.pos_2.y));
                            path_builder.line_to(Vec2::new(
                                ae.pos_2.x + h + cover_line_w * 2.0,
                                ae.pos_2.y,
                            ));
                            path_builder.line_to(Vec2::new(
                                ae.pos_2.x + h + cover_line_w * 2.0,
                                ae.pos_1.y,
                            ));
                            path_builder.close();
                            parent.spawn((
                                ShapeBundle {
                                    path: path_builder.build(),
                                    ..default()
                                },
                                Stroke::new(BG_COLOR, cover_line_w),
                                Fill::color(BG_COLOR),
                            ));
                        });
                }
            });
            if ae.delta == 1.0 {
                ae.is_done = true;
            }
        };
    }
}
