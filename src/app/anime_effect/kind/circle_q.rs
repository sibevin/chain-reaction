use super::*;
use crate::app::theme::*;

pub struct AnimeEffectKindCircleQ;

impl AnimeEffectKindBase for AnimeEffectKindCircleQ {
    fn create(&self, commands: &mut Commands, param: AnimeEffectParam) -> Entity {
        let root_entity = commands
            .spawn((SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, ANIME_EFFECT_Z_INDEX),
                ..default()
            },))
            .id();
        let segments: Vec<[Vec2; 4]> = build_segements(param.pos_1, param.pos_2);
        let ae = AnimeEffect {
            kind: param.kind,
            segments,
            pos_1: param.pos_1,
            pos_2: param.pos_2,
            layer: 0,
            delta: 0.0,
            color: param.color,
            width: param.width_start,
            radius: (param.pos_2 - param.pos_1).length() * 2.0_f32.sqrt() + param.width_start * 2.0,
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
        if ae.delta < 1.0 {
            ae.layer += 1;
            if let Some(mut entity_commands) = commands.get_entity(ae.root_entity) {
                // entity_commands.despawn_descendants();
                entity_commands.with_children(|parent| {
                    let mut last_pos = Vec2::default();
                    if !ae.segments.is_empty() {
                        parent
                            .spawn((SpatialBundle {
                                transform: Transform::from_xyz(
                                    0.0,
                                    0.0,
                                    ae.layer as f32 * 0.001 + 0.0001,
                                ),
                                ..default()
                            },))
                            .with_children(|parent| {
                                for segment in ae.segments.iter() {
                                    let mut path_builder = PathBuilder::new();
                                    path_builder.move_to(segment[0]);
                                    path_builder.quadratic_bezier_to(segment[1], segment[2]);
                                    parent.spawn((
                                        ShapeBundle {
                                            path: path_builder.build(),
                                            ..default()
                                        },
                                        Stroke::new(ae.color, ae.width),
                                    ));
                                    let circle = shapes::Circle {
                                        radius: ae.width / 2.0,
                                        center: segment[0],
                                    };
                                    let geo_builder = GeometryBuilder::new().add(&circle);
                                    parent.spawn((
                                        ShapeBundle {
                                            path: geo_builder.build(),
                                            ..default()
                                        },
                                        Fill::color(ae.color),
                                    ));
                                    last_pos = segment[2];
                                }
                            });
                        let start_angle = -PI / 2.0;
                        let angle = PI * 1.98 * ae.delta;
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
                                    transform: Transform::from_xyz(
                                        ae.pos_1.x,
                                        ae.pos_1.y,
                                        ae.layer as f32 * 0.001 + 0.0002,
                                    )
                                    .with_rotation(
                                        Quat::from_rotation_z(start_angle - angle * 2.0),
                                    ),
                                    ..default()
                                },
                                ..default()
                            },
                            Fill::color(BG_COLOR),
                        ));
                        if ae.delta > 0.98 {
                            let circle = shapes::Circle {
                                radius: ae.width / 2.0,
                                center: last_pos,
                            };
                            let geo_builder = GeometryBuilder::new().add(&circle);
                            parent.spawn((
                                ShapeBundle {
                                    path: geo_builder.build(),
                                    spatial: SpatialBundle {
                                        transform: Transform::from_xyz(
                                            0.0,
                                            0.0,
                                            ae.layer as f32 * 0.001 + 0.0002,
                                        ),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Fill::color(ae.color),
                            ));
                        }
                    }
                });
            };
        }
    }
}

const DRAW_START_ANGLE: f32 = -PI / 9.0;

fn build_segements(pos_1: Vec2, pos_2: Vec2) -> Vec<[Vec2; 4]> {
    let mut segments: Vec<[Vec2; 4]> = vec![];
    let circle_vec = Vec2::new(0.0, -1.0);
    let radius = (pos_2 - pos_1).length();
    let noise_radius = radius * 0.2;
    let mut prev_pos = noise_pos(
        pos_1 + Vec2::from_angle(DRAW_START_ANGLE).rotate(circle_vec) * radius / 2.0_f32.sqrt(),
        noise_radius,
    );
    let start_angle = -PI / 4.0;
    let mut prev_ctrl = noise_pos(
        pos_1 + Vec2::from_angle(start_angle).rotate(circle_vec) * radius,
        noise_radius,
    );
    for i in 0..4 {
        let i = i as f32;
        let mut angle = start_angle - PI / 2.0;
        let ctrl_pos_1 = prev_ctrl;
        angle -= (PI / 2.0) * 0.95 * i;
        let ctrl_pos_2 = noise_pos(
            pos_1 + Vec2::from_angle(angle).rotate(circle_vec) * radius,
            noise_radius,
        );
        let middle_pos = (ctrl_pos_1 + ctrl_pos_2) / 2.0;
        segments.push([prev_pos, ctrl_pos_1, middle_pos, Vec2::ZERO]);
        prev_pos = middle_pos;
        prev_ctrl = ctrl_pos_2;
    }
    segments
}
