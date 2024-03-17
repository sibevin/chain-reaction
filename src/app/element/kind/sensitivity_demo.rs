use super::*;
use crate::app::{interaction, theme, ui};

const SD_SIZE: f32 = ui::FONT_SIZE * 7.0;
const SD_P: f32 = ui::FONT_SIZE * 0.5;
const SD_CONTROL_SIZE: f32 = SD_SIZE - SD_P * 2.0;
const SD_W: f32 = ui::FONT_SIZE * 0.3;
const SD_LINE_W: f32 = ui::FONT_SIZE * 0.045;

pub fn build_element(
    parent: &mut ChildBuilder,
    bundle: impl Bundle,
    default_sensitivity: ElementTargetValuePair,
    modified_sensitivity: ElementTargetValuePair,
) -> Entity {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(SD_SIZE),
                    height: Val::Px(SD_SIZE),
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: theme::BTN_BG.into(),
                ..default()
            },
            bundle,
            interaction::IaCrossPanel,
            Focusable::default(),
            ElementData::SensitivityDemo {
                default_sensitivity,
                modified_sensitivity,
                ball_pos: Vec2::ZERO,
                is_modifier_on: false,
                is_locked: false,
            },
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
            let half_control_size = SD_CONTROL_SIZE / 2.0;
            let corners: [Vec2; 4] = [
                center_pos + Vec2::new(half_control_size, half_control_size),
                center_pos + Vec2::new(half_control_size, -half_control_size),
                center_pos + Vec2::new(-half_control_size, -half_control_size),
                center_pos + Vec2::new(-half_control_size, half_control_size),
            ];
            let corner_bevel_size = SD_W / 2.0 * 0.6;
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
                if i % 2 == 1 {
                    path_builder.quadratic_bezier_to(corners[(i - 1) / 2], panel_poses[i]);
                } else {
                    path_builder.line_to(panel_poses[i]);
                }
            }
            path_builder.close();
            parent.spawn((
                ShapeBundle {
                    path: path_builder.build(),
                    ..default()
                },
                Fill::color(theme::MUTE_COLOR.with_a(0.6)),
            ));
        });
    }
}

pub fn update_display(
    commands: &mut Commands,
    window: &Query<&Window>,
    g_trans: &GlobalTransform,
    dyn_entity: Entity,
    ball_pos: &Vec2,
    is_modifier_on: &bool,
    is_locked: &bool,
) {
    if let Some(mut entity_commands) = commands.get_entity(dyn_entity) {
        entity_commands.with_children(|parent| {
            let center_pos = fetch_center_pos(&window, &g_trans);
            let circle = shapes::Circle {
                radius: SD_W / 2.0,
                center: *ball_pos + center_pos,
            };
            let geo_builder = GeometryBuilder::new().add(&circle);
            parent.spawn((
                ShapeBundle {
                    path: geo_builder.build(),
                    ..default()
                },
                Fill::color(theme::FG_COLOR),
            ));
            if *is_locked {
                let mut path_builder = PathBuilder::new();
                path_builder.move_to(center_pos + Vec2::new(0.0, (SD_CONTROL_SIZE - SD_W) / 2.0));
                path_builder.line_to(center_pos + Vec2::new(0.0, (-SD_CONTROL_SIZE + SD_W) / 2.0));
                path_builder.move_to(center_pos + Vec2::new((SD_CONTROL_SIZE - SD_W) / 2.0, 0.0));
                path_builder.line_to(center_pos + Vec2::new((-SD_CONTROL_SIZE + SD_W) / 2.0, 0.0));
                parent.spawn((
                    ShapeBundle {
                        path: path_builder.build(),
                        ..default()
                    },
                    Stroke::new(theme::FG_COLOR, SD_LINE_W),
                ));
            }
            if *is_modifier_on {
                let dot_poses: [Vec2; 5] = [
                    center_pos,
                    center_pos + Vec2::new(0.0, (SD_CONTROL_SIZE - SD_W) / 2.0),
                    center_pos + Vec2::new(0.0, (-SD_CONTROL_SIZE + SD_W) / 2.0),
                    center_pos + Vec2::new((SD_CONTROL_SIZE - SD_W) / 2.0, 0.0),
                    center_pos + Vec2::new((-SD_CONTROL_SIZE + SD_W) / 2.0, 0.0),
                ];
                for dot_pos in dot_poses {
                    let circle = shapes::Circle {
                        radius: SD_LINE_W * 2.0,
                        center: dot_pos,
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
        });
    }
}

const MOVING_RATIO: f32 = 0.05;

pub fn handle_mouse_dragging(
    motion_events: &mut EventReader<input::mouse::MouseMotion>,
    default_sensitivity: &ElementTargetValuePair,
    modified_sensitivity: &ElementTargetValuePair,
    is_modifier_on: &bool,
    ball_pos: &mut Vec2,
) {
    let sensitivity = if *is_modifier_on {
        modified_sensitivity.u8_value.unwrap()
    } else {
        default_sensitivity.u8_value.unwrap()
    };
    let motion_events = motion_events.read().collect::<Vec<_>>();
    if let Some(motion_event) = motion_events.iter().next() {
        let value_delta = Vec2::new(motion_event.delta.x, -motion_event.delta.y)
            * sensitivity as f32
            * MOVING_RATIO;
        *ball_pos = (*ball_pos + value_delta).clamp(
            Vec2::new(-SD_CONTROL_SIZE / 2.0, -SD_CONTROL_SIZE / 2.0),
            Vec2::new(SD_CONTROL_SIZE / 2.0, SD_CONTROL_SIZE / 2.0),
        );
    }
}

pub fn handle_element_changing(
    key_action: &ElementAction,
    delta: f32,
    default_sensitivity: &ElementTargetValuePair,
    modified_sensitivity: &ElementTargetValuePair,
    ball_pos: &mut Vec2,
    is_modifier_on: &bool,
    is_locked: &bool,
) {
    if *is_locked {
        let sensitivity = if *is_modifier_on {
            modified_sensitivity.u8_value.unwrap()
        } else {
            default_sensitivity.u8_value.unwrap()
        };
        let delta_v: Vec2 = match key_action {
            ElementAction::Right => Vec2::new(delta, 0.0),
            ElementAction::Left => Vec2::new(-delta, 0.0),
            ElementAction::Up => Vec2::new(0.0, delta),
            ElementAction::Down => Vec2::new(0.0, -delta),
            _ => Vec2::ZERO,
        };
        if delta_v.length() > 0.0 {
            let value_delta = delta_v * sensitivity as f32 * MOVING_RATIO;
            *ball_pos = (*ball_pos + value_delta).clamp(
                Vec2::new(-SD_CONTROL_SIZE / 2.0, -SD_CONTROL_SIZE / 2.0),
                Vec2::new(SD_CONTROL_SIZE / 2.0, SD_CONTROL_SIZE / 2.0),
            );
        }
    }
}

fn fetch_center_pos(window: &Query<&Window>, g_trans: &GlobalTransform) -> Vec2 {
    let world_pos = Vec2::new(g_trans.translation().x, g_trans.translation().y);
    to_canvas_pos(&window, world_pos)
}
