use super::{round_to_five, AppUiData, AppUiTargetValuePair};
use crate::app::ui::*;
use bevy::input;

const X_PANEL_SIZE: f32 = FONT_SIZE * 7.0;
const X_PANEL_P: f32 = FONT_SIZE * 0.5;
const X_PANEL_CONTROL_SIZE: f32 = X_PANEL_SIZE - X_PANEL_P * 2.0;
const X_PANEL_W: f32 = FONT_SIZE * 0.5;
const X_PANEL_B: f32 = FONT_SIZE * 0.1;
const MARK_COUNT: u8 = 10;

pub fn build_cross_panel_ui(
    canvas_em: AppUiCanvasEntityMap,
    parent: &mut ChildBuilder,
    bundle: impl Bundle,
    x: AppUiTargetValuePair,
    y: AppUiTargetValuePair,
) -> Entity {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(X_PANEL_SIZE),
                    height: Val::Px(X_PANEL_SIZE),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BTN_BG.into(),
                ..default()
            },
            bundle,
            app::interaction::IaCrossPanel,
            AppUiData::CrossPanel {
                x,
                y,
                canvas_em,
                is_modifier_on: false,
                is_locked: false,
            },
            Focusable::default(),
        ))
        .id()
}

pub fn init_ui_display(
    commands: &mut Commands,
    window: &Query<&Window>,
    g_trans: &GlobalTransform,
    canvas_em: &AppUiCanvasEntityMap,
) {
    if let Some(mut entity_commands) = commands.get_entity(canvas_em.bg) {
        entity_commands.despawn_descendants();
        entity_commands.with_children(|parent| {
            let center_pos = fetch_center_pos(&window, &g_trans);
            let half_control_size = X_PANEL_CONTROL_SIZE / 2.0;
            let corners: [Vec2; 4] = [
                center_pos + Vec2::new(half_control_size, half_control_size),
                center_pos + Vec2::new(half_control_size, -half_control_size),
                center_pos + Vec2::new(-half_control_size, -half_control_size),
                center_pos + Vec2::new(-half_control_size, half_control_size),
            ];
            let corner_bevel_size = X_PANEL_W / 2.0 * 0.6;
            let panel_poses: [Vec2; 8] = [
                corners[0] + Vec2::new(-corner_bevel_size, 0.0),
                corners[0] + Vec2::new(0.0, -corner_bevel_size),
                corners[1] + Vec2::new(0.0, corner_bevel_size),
                corners[1] + Vec2::new(-corner_bevel_size, 0.0),
                corners[2] + Vec2::new(corner_bevel_size, 0.0),
                corners[2] + Vec2::new(0.0, corner_bevel_size),
                corners[3] + Vec2::new(0.0, -corner_bevel_size),
                corners[3] + Vec2::new(corner_bevel_size, 0.0),
            ];
            let mut path_builder = PathBuilder::new();
            path_builder.move_to(panel_poses[0]);
            for i in 1..8 {
                path_builder.line_to(panel_poses[i]);
            }
            path_builder.close();
            parent.spawn((
                ShapeBundle {
                    path: path_builder.build(),
                    ..default()
                },
                Stroke::new(SECONDARY_COLOR, X_PANEL_W),
                Fill::color(SECONDARY_COLOR),
            ));
            for corner in corners {
                let circle = shapes::Circle {
                    radius: X_PANEL_W / 2.0,
                    center: corner,
                };
                let geo_builder = GeometryBuilder::new().add(&circle);
                parent.spawn((
                    ShapeBundle {
                        path: geo_builder.build(),
                        ..default()
                    },
                    Fill::color(SECONDARY_COLOR),
                ));
            }
        });
    }
}

const DOT_SIZE: f32 = X_PANEL_CONTROL_SIZE * 0.0022;

pub fn update_ui_display(
    commands: &mut Commands,
    window: &Query<&Window>,
    g_trans: &GlobalTransform,
    canvas_em: &AppUiCanvasEntityMap,
    x: &AppUiTargetValuePair,
    y: &AppUiTargetValuePair,
    is_modifier_on: &bool,
    is_locked: &bool,
) {
    if let Some(mut entity_commands) = commands.get_entity(canvas_em.fg) {
        entity_commands.despawn_descendants();
        entity_commands.with_children(|parent| {
            let center_pos = fetch_center_pos(&window, &g_trans);
            let thumb_pos = center_pos
                + Vec2::new(
                    (x.value as f32 - 50.0) / 100.0 * X_PANEL_CONTROL_SIZE,
                    (y.value as f32 - 50.0) / 100.0 * X_PANEL_CONTROL_SIZE,
                );
            let circle = shapes::Circle {
                radius: X_PANEL_W / 2.0 - X_PANEL_B,
                center: thumb_pos,
            };
            let geo_builder = GeometryBuilder::new().add(&circle);
            parent.spawn((
                ShapeBundle {
                    path: geo_builder.build(),
                    ..default()
                },
                Fill::color(FG_COLOR),
            ));
            if *is_modifier_on || *is_locked {
                let start_pos = center_pos - Vec2::ONE * X_PANEL_CONTROL_SIZE * 0.5;
                let mut path_builder = PathBuilder::new();
                path_builder.move_to(start_pos + Vec2::new(0.0, X_PANEL_CONTROL_SIZE / 2.0));
                path_builder.line_to(
                    start_pos + Vec2::new(X_PANEL_CONTROL_SIZE, X_PANEL_CONTROL_SIZE / 2.0),
                );
                path_builder.move_to(start_pos + Vec2::new(X_PANEL_CONTROL_SIZE / 2.0, 0.0));
                path_builder.line_to(
                    start_pos + Vec2::new(X_PANEL_CONTROL_SIZE / 2.0, X_PANEL_CONTROL_SIZE),
                );
                parent.spawn((
                    ShapeBundle {
                        path: path_builder.build(),
                        ..default()
                    },
                    Stroke::new(FG_COLOR, DOT_SIZE * 3.0),
                ));
            }

            if *is_modifier_on {
                let start_pos = center_pos - Vec2::ONE * X_PANEL_CONTROL_SIZE * 0.5;
                let panel_v_size = X_PANEL_CONTROL_SIZE / (MARK_COUNT as f32);
                for i in 0..=MARK_COUNT {
                    for j in 0..=MARK_COUNT {
                        let mark_pos = start_pos
                            + Vec2::new(panel_v_size * (i as f32), panel_v_size * (j as f32));
                        let circle = shapes::Circle {
                            radius: DOT_SIZE * 5.0,
                            center: mark_pos,
                        };
                        let geo_builder = GeometryBuilder::new().add(&circle);
                        parent.spawn((
                            ShapeBundle {
                                path: geo_builder.build(),
                                ..default()
                            },
                            Fill::color(FG_COLOR),
                        ));
                        if i > 0 && j > 0 {
                            let mark_pos = start_pos
                                + Vec2::new(
                                    panel_v_size * (i as f32 - 0.5),
                                    panel_v_size * (j as f32 - 0.5),
                                );
                            let circle = shapes::Circle {
                                radius: DOT_SIZE * 3.0,
                                center: mark_pos,
                            };
                            let geo_builder = GeometryBuilder::new().add(&circle);
                            parent.spawn((
                                ShapeBundle {
                                    path: geo_builder.build(),
                                    ..default()
                                },
                                Fill::color(FG_COLOR),
                            ));
                        }
                        if j > 0 {
                            let mark_pos = start_pos
                                + Vec2::new(
                                    panel_v_size * (i as f32),
                                    panel_v_size * (j as f32 - 0.5),
                                );
                            let circle = shapes::Circle {
                                radius: DOT_SIZE * 3.0,
                                center: mark_pos,
                            };
                            let geo_builder = GeometryBuilder::new().add(&circle);
                            parent.spawn((
                                ShapeBundle {
                                    path: geo_builder.build(),
                                    ..default()
                                },
                                Fill::color(FG_COLOR),
                            ));
                        }
                        if i > 0 {
                            let mark_pos = start_pos
                                + Vec2::new(
                                    panel_v_size * (i as f32 - 0.5),
                                    panel_v_size * (j as f32),
                                );
                            let circle = shapes::Circle {
                                radius: DOT_SIZE * 3.0,
                                center: mark_pos,
                            };
                            let geo_builder = GeometryBuilder::new().add(&circle);
                            parent.spawn((
                                ShapeBundle {
                                    path: geo_builder.build(),
                                    ..default()
                                },
                                Fill::color(FG_COLOR),
                            ));
                        }
                    }
                }
            }
        });
    }
}

pub fn handle_mouse_clicking(
    window: &Query<&Window>,
    g_trans: &GlobalTransform,
    cursor_data: &Res<app::cursor::AppCursorData>,
    x: &mut AppUiTargetValuePair,
    y: &mut AppUiTargetValuePair,
    is_modifier_on: &bool,
) {
    let center_pos = fetch_center_pos(&window, &g_trans);
    let value_pos = (Vec2::new(50.0, 50.0)
        + (cursor_data.canvas_pos - center_pos) / X_PANEL_CONTROL_SIZE * 100.0)
        .clamp(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0));
    x.value = round_to_five(value_pos.x as u8, *is_modifier_on);
    y.value = round_to_five(value_pos.y as u8, *is_modifier_on);
}

pub fn handle_mouse_dragging(
    motion_events: &mut EventReader<input::mouse::MouseMotion>,
    x: &mut AppUiTargetValuePair,
    y: &mut AppUiTargetValuePair,
    is_modifier_on: &bool,
) {
    let dragging_moving_ratio: f32 = if *is_modifier_on { 2.0 } else { 1.0 };
    let motion_events = motion_events.read().collect::<Vec<_>>();
    if let Some(motion_event) = motion_events.iter().rev().take(3).next() {
        let value_delta =
            Vec2::new(motion_event.delta.x, -motion_event.delta.y) * dragging_moving_ratio;
        x.value = round_to_five(
            (x.value as f32 + value_delta.x).clamp(0.0, 100.0) as u8,
            *is_modifier_on,
        );
        y.value = round_to_five(
            (y.value as f32 + value_delta.y).clamp(0.0, 100.0) as u8,
            *is_modifier_on,
        );
    }
}

fn fetch_center_pos(window: &Query<&Window>, g_trans: &GlobalTransform) -> Vec2 {
    let world_pos = Vec2::new(g_trans.translation().x, g_trans.translation().y);
    kind::to_canvas_pos(&window, world_pos)
}
