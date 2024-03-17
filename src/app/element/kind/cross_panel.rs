use super::*;
use crate::app::{cursor, interaction, theme, ui};
use bevy::input;

const X_PANEL_SIZE: f32 = ui::FONT_SIZE * 7.0;
const X_PANEL_P: f32 = ui::FONT_SIZE * 0.5;
const X_PANEL_CONTROL_SIZE: f32 = X_PANEL_SIZE - X_PANEL_P * 2.0;
const X_PANEL_W: f32 = ui::FONT_SIZE * 0.5;
const X_PANEL_B: f32 = ui::FONT_SIZE * 0.1;
const MARK_COUNT: u8 = 10;

pub fn build_element(
    parent: &mut ChildBuilder,
    bundle: impl Bundle,
    x: ElementTargetValuePair,
    y: ElementTargetValuePair,
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
                background_color: theme::BTN_BG.into(),
                ..default()
            },
            bundle,
            interaction::IaCrossPanel,
            ElementData::CrossPanel {
                x,
                y,
                is_modifier_on: false,
                is_locked: false,
            },
            Focusable::default(),
        ))
        .id()
}

pub fn init_display(
    commands: &mut Commands,
    window: &Query<&Window>,
    g_trans: &GlobalTransform,
    fg_entity: Entity,
) {
    if let Some(mut entity_commands) = commands.get_entity(fg_entity) {
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
                Stroke::new(theme::SECONDARY_COLOR, X_PANEL_W),
                Fill::color(theme::SECONDARY_COLOR),
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
                    Fill::color(theme::SECONDARY_COLOR),
                ));
            }
        });
    }
}

const DOT_SIZE: f32 = X_PANEL_CONTROL_SIZE * 0.0022;

pub fn update_display(
    commands: &mut Commands,
    window: &Query<&Window>,
    g_trans: &GlobalTransform,
    dyn_entity: Entity,
    x: &ElementTargetValuePair,
    y: &ElementTargetValuePair,
    is_modifier_on: &bool,
    is_locked: &bool,
) {
    let x_value = x.u8_value.unwrap();
    let y_value = y.u8_value.unwrap();
    if let Some(mut entity_commands) = commands.get_entity(dyn_entity) {
        entity_commands.with_children(|parent| {
            let center_pos = fetch_center_pos(&window, &g_trans);
            let thumb_pos = center_pos
                + Vec2::new(
                    (x_value as f32 - 50.0) / 100.0 * X_PANEL_CONTROL_SIZE,
                    (y_value as f32 - 50.0) / 100.0 * X_PANEL_CONTROL_SIZE,
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
                Fill::color(theme::FG_COLOR),
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
                    Stroke::new(theme::FG_COLOR, DOT_SIZE * 3.0),
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
                            Fill::color(theme::FG_COLOR),
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
                                Fill::color(theme::FG_COLOR),
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
                                Fill::color(theme::FG_COLOR),
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
                                Fill::color(theme::FG_COLOR),
                            ));
                        }
                    }
                }
            }
        });
    }
}

pub fn handle_mouse_pressing(
    window: &Query<&Window>,
    g_trans: &GlobalTransform,
    cursor_data: &Res<cursor::AppCursorData>,
    x: &mut ElementTargetValuePair,
    y: &mut ElementTargetValuePair,
    is_modifier_on: &bool,
) {
    let center_pos = fetch_center_pos(&window, &g_trans);
    let value_pos = (Vec2::new(50.0, 50.0)
        + (cursor_data.canvas_pos - center_pos) / X_PANEL_CONTROL_SIZE * 100.0)
        .clamp(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0));
    x.u8_value = Some(round_to_five(value_pos.x as u8, *is_modifier_on));
    y.u8_value = Some(round_to_five(value_pos.y as u8, *is_modifier_on));
}

pub fn handle_mouse_dragging(
    motion_events: &mut EventReader<input::mouse::MouseMotion>,
    x: &mut ElementTargetValuePair,
    y: &mut ElementTargetValuePair,
    is_modifier_on: &bool,
) {
    let x_value = x.u8_value.unwrap();
    let y_value = y.u8_value.unwrap();
    let dragging_moving_ratio: f32 = if *is_modifier_on { 2.0 } else { 1.0 };
    let motion_events = motion_events.read().collect::<Vec<_>>();
    if let Some(motion_event) = motion_events.iter().rev().take(3).next() {
        let value_delta =
            Vec2::new(motion_event.delta.x, -motion_event.delta.y) * dragging_moving_ratio;
        x.u8_value = Some(round_to_five(
            (x_value as f32 + value_delta.x).clamp(0.0, 100.0) as u8,
            *is_modifier_on,
        ));
        y.u8_value = Some(round_to_five(
            (y_value as f32 + value_delta.y).clamp(0.0, 100.0) as u8,
            *is_modifier_on,
        ));
    }
}

pub fn handle_element_changing(
    key_action: &ElementAction,
    x: &mut ElementTargetValuePair,
    y: &mut ElementTargetValuePair,
    is_modifier_on: &bool,
    is_locked: &bool,
    event_writer: &mut EventWriter<ElementEvent>,
) {
    if *is_locked {
        let mut change: (String, i8) = (String::from("main"), 1);
        match key_action {
            ElementAction::Right => {
                change = (String::from("main"), 1);
            }
            ElementAction::Left => {
                change = (String::from("main"), -1);
            }
            ElementAction::Up => {
                change = (String::from("sub"), 1);
            }
            ElementAction::Down => {
                change = (String::from("sub"), -1);
            }
            _ => (),
        }
        if change.0 == "main" {
            let ori_x = x.u8_value.unwrap();
            x.u8_value = Some(calculate_changed_value(ori_x, change.1, *is_modifier_on));
            if ori_x != x.u8_value.unwrap() {
                event_writer.send(ElementEvent::DataChanged { data: x.clone() });
            }
        }
        if change.0 == "sub" {
            let ori_y = y.u8_value.unwrap();
            y.u8_value = Some(calculate_changed_value(ori_y, change.1, *is_modifier_on));
            if ori_y != y.u8_value.unwrap() {
                event_writer.send(ElementEvent::DataChanged { data: y.clone() });
            }
        }
    }
}

fn fetch_center_pos(window: &Query<&Window>, g_trans: &GlobalTransform) -> Vec2 {
    let world_pos = Vec2::new(g_trans.translation().x, g_trans.translation().y);
    to_canvas_pos(&window, world_pos)
}
