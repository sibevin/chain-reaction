use super::*;
use crate::app::*;
use bevy::input;

const SLIDER_BAR_H: f32 = ui::FONT_SIZE * 0.5;
const SLIDER_BAR_W: f32 = ui::FONT_SIZE * 7.0;
const SLIDER_BAR_B: f32 = ui::FONT_SIZE * 0.1;
const SLIDER_TEXT_W: f32 = ui::FONT_SIZE * 2.5;
const SLIDER_P: f32 = ui::FONT_SIZE * 1.0;
const SLIDER_W: f32 = SLIDER_P * 2.0 + SLIDER_BAR_W + SLIDER_TEXT_W;
const SLIDER_H: f32 = ui::FONT_SIZE * 2.0;
const MARK_COUNT: u8 = 10;

pub fn build_element(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    bundle: impl Bundle,
    data: ElementTargetValuePair,
) -> Entity {
    let value = data.u8_value.unwrap_or(200);
    if value > 100 {
        panic!("Invalid slider value");
    }
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(SLIDER_W),
                    height: Val::Px(SLIDER_H),
                    padding: UiRect::right(Val::Px(SLIDER_P)),
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: theme::BTN_BG.into(),
                ..default()
            },
            bundle,
            interaction::IaSlider,
            ElementData::Slider {
                data,
                is_modifier_on: false,
                is_locked: false,
            },
            Focusable::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    format!("{}", value),
                    TextStyle {
                        font: asset_server.load(theme::FONT),
                        font_size: ui::FONT_SIZE,
                        color: theme::FG_COLOR,
                    },
                ),
                ElementText,
            ));
        })
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
            let (bar_start_pos, bar_end_pos) = fetch_bar_pos(&window, &g_trans);
            let mut path_builder = PathBuilder::new();
            path_builder.move_to(bar_start_pos);
            path_builder.line_to(bar_end_pos);
            parent.spawn((
                ShapeBundle {
                    path: path_builder.build(),
                    ..default()
                },
                Stroke::new(theme::SECONDARY_COLOR, SLIDER_BAR_H),
            ));
            let circle = shapes::Circle {
                radius: SLIDER_BAR_H / 2.0,
                center: bar_start_pos,
            };
            let geo_builder = GeometryBuilder::new().add(&circle);
            parent.spawn((
                ShapeBundle {
                    path: geo_builder.build(),
                    ..default()
                },
                Fill::color(theme::SECONDARY_COLOR),
            ));
            let circle = shapes::Circle {
                radius: SLIDER_BAR_H / 2.0,
                center: bar_end_pos,
            };
            let geo_builder = GeometryBuilder::new().add(&circle);
            parent.spawn((
                ShapeBundle {
                    path: geo_builder.build(),
                    ..default()
                },
                Fill::color(theme::SECONDARY_COLOR),
            ));
        });
    }
}

pub fn update_text(ui_text: &mut Text, data: &ElementTargetValuePair) {
    let value = data.u8_value.unwrap_or(200);
    if value > 100 {
        panic!("Invalid slider value");
    }
    ui_text.sections[0].value = format!("{}", value);
}

pub fn update_display(
    commands: &mut Commands,
    window: &Query<&Window>,
    g_trans: &GlobalTransform,
    dyn_entity: Entity,
    data: &ElementTargetValuePair,
    is_modifier_on: &bool,
    is_locked: &bool,
) {
    let value = data.u8_value.unwrap();
    if let Some(mut entity_commands) = commands.get_entity(dyn_entity) {
        entity_commands.with_children(|parent| {
            let (bar_start_pos, bar_end_pos) = fetch_bar_pos(&window, &g_trans);
            let bar_thumb_pos = fetch_thumb_pos(value, bar_start_pos, bar_end_pos);
            let mut path_builder = PathBuilder::new();
            path_builder.move_to(bar_start_pos);
            path_builder.line_to(bar_thumb_pos);
            parent.spawn((
                ShapeBundle {
                    path: path_builder.build(),
                    ..default()
                },
                Stroke::new(theme::FG_COLOR, SLIDER_BAR_H - SLIDER_BAR_B * 2.0),
            ));
            let circle = shapes::Circle {
                radius: SLIDER_BAR_H / 2.0 - SLIDER_BAR_B,
                center: bar_start_pos,
            };
            let geo_builder = GeometryBuilder::new().add(&circle);
            parent.spawn((
                ShapeBundle {
                    path: geo_builder.build(),
                    ..default()
                },
                Fill::color(theme::FG_COLOR),
            ));
            let circle = shapes::Circle {
                radius: SLIDER_BAR_H / 2.0 - SLIDER_BAR_B,
                center: bar_thumb_pos,
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
                path_builder.move_to(bar_start_pos);
                path_builder.line_to(bar_end_pos);
                parent.spawn((
                    ShapeBundle {
                        path: path_builder.build(),
                        ..default()
                    },
                    Stroke::new(theme::FG_COLOR, (SLIDER_BAR_H / 2.0 - SLIDER_BAR_B) * 0.3),
                ));
            }
            if *is_modifier_on {
                let bar_v = (bar_end_pos - bar_start_pos) / (MARK_COUNT as f32);
                for i in 1..=MARK_COUNT {
                    let mark_pos = bar_start_pos + bar_v * (i as f32);
                    let circle = shapes::Circle {
                        radius: (SLIDER_BAR_H / 2.0 - SLIDER_BAR_B) * 0.5,
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
                    let mark_pos = bar_start_pos + bar_v * (i as f32 - 0.5);
                    let circle = shapes::Circle {
                        radius: (SLIDER_BAR_H / 2.0 - SLIDER_BAR_B) * 0.3,
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
        });
    }
}

pub fn handle_mouse_clicking(
    window: &Query<&Window>,
    g_trans: &GlobalTransform,
    cursor_data: &Res<cursor::AppCursorData>,
    data: &mut ElementTargetValuePair,
    is_modifier_on: &bool,
) {
    let (bar_start_pos, _) = fetch_bar_pos(&window, &g_trans);
    let value = ((cursor_data.canvas_pos.x - bar_start_pos.x) / SLIDER_BAR_W * 100.0)
        .clamp(0.0, 100.0) as u8;
    data.u8_value = Some(round_to_five(value, *is_modifier_on));
}

pub fn handle_mouse_dragging(
    motion_events: &mut EventReader<input::mouse::MouseMotion>,
    data: &mut ElementTargetValuePair,
    is_modifier_on: &bool,
) {
    let value = data.u8_value.unwrap();
    let dragging_moving_ratio: f32 = if *is_modifier_on { 2.0 } else { 0.8 };
    let motion_events = motion_events.read().collect::<Vec<_>>();
    if let Some(motion_event) = motion_events.iter().rev().take(3).next() {
        let new_value = (value as i8 + (motion_event.delta.x * dragging_moving_ratio) as i8)
            .clamp(0, 100) as u8;
        data.u8_value = Some(round_to_five(new_value, *is_modifier_on));
    }
}

fn fetch_bar_pos(window: &Query<&Window>, g_trans: &GlobalTransform) -> (Vec2, Vec2) {
    let world_pos = Vec2::new(g_trans.translation().x, g_trans.translation().y);
    let g_pos = to_canvas_pos(&window, world_pos);
    let bar_start_pos = Vec2::new(g_pos.x + SLIDER_P - SLIDER_W / 2.0, g_pos.y);
    let bar_end_pos = Vec2::new(bar_start_pos.x + SLIDER_BAR_W, bar_start_pos.y);
    (bar_start_pos, bar_end_pos)
}

fn fetch_thumb_pos(value: u8, bar_start_pos: Vec2, bar_end_pos: Vec2) -> Vec2 {
    let ratio = value as f32 / 100.0;
    bar_start_pos + (bar_end_pos - bar_start_pos) * ratio
}
