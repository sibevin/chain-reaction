use super::*;
use crate::app::{theme::*, ui};

pub struct AnimeEffectKindLineQ;

const LINE_SEGMENT: f32 = ui::FONT_SIZE * 2.0;

impl AnimeEffectKindBase for AnimeEffectKindLineQ {
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
            radius: (param.pos_2.y - param.pos_1.y).abs() + param.width_start * 2.0,
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
                        let first_pos = ae.segments.first().unwrap()[0];
                        let last_pos = ae.segments.last().unwrap()[2];
                        let line_vec = last_pos - first_pos;
                        let start_pos = first_pos + line_vec * ae.delta;
                        let line =
                            shapes::Line(start_pos, last_pos + line_vec.normalize() * ae.radius);
                        let path_builder = GeometryBuilder::new().add(&line);
                        parent.spawn((
                            ShapeBundle {
                                path: path_builder.build(),
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
                            Stroke::new(BG_COLOR, ae.radius),
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
                                            ae.layer as f32 * 0.001 + 0.0003,
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

fn build_segements(pos_1: Vec2, pos_2: Vec2) -> Vec<[Vec2; 4]> {
    let mut segments: Vec<[Vec2; 4]> = vec![];
    let line_length = pos_2.x - pos_1.x;
    let line_width = (pos_2.y - pos_1.y) * 0.7;
    let wave_count = (line_length / LINE_SEGMENT).ceil() as usize;
    let wave_length = line_length / wave_count as f32;
    let noise_radius = line_width.abs() * 0.5;
    let mut prev_pos = noise_pos(Vec2::new(pos_1.x, pos_1.y + line_width / 2.0), noise_radius);
    for i in 0..wave_count {
        let i = i as f32;
        let ctrl_pos_1 = noise_pos(
            Vec2::new(pos_1.x + wave_length * (i + 0.25), pos_1.y + line_width),
            noise_radius,
        );
        let ctrl_pos_2 = noise_pos(
            Vec2::new(pos_1.x + wave_length * (i + 0.75), pos_1.y),
            noise_radius,
        );
        let middle_pos = (ctrl_pos_1 + ctrl_pos_2) / 2.0;
        let end_pos = noise_pos(
            Vec2::new(
                pos_1.x + wave_length * (i + 1.0),
                pos_1.y + line_width / 2.0,
            ),
            noise_radius,
        );
        segments.push([prev_pos, ctrl_pos_1, middle_pos, Vec2::ZERO]);
        segments.push([middle_pos, ctrl_pos_2, end_pos, Vec2::ZERO]);
        prev_pos = end_pos;
    }
    segments
}
