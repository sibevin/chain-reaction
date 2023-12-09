use crate::{app, reactor};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub mod alpha_count;
pub mod score;
pub mod timer;

const FIELD_TEXT_SIZE: f32 = reactor::FIELD_NAV_H * 0.5;
const FIELD_PADDING: f32 = (reactor::FIELD_NAV_H - FIELD_TEXT_SIZE) / 2.0;

const FIELD_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
const FIELD_TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

#[derive(Component)]
pub struct FieldTimer(pub u32);

#[derive(Component)]
pub struct FieldAlphaCount(pub u32);

#[derive(Component)]
pub struct FieldScore(pub u32);

pub fn get_field_rect() -> Rect {
    Rect::new(
        -reactor::FIELD_W / 2.0,
        (-reactor::FIELD_H + reactor::FIELD_NAV_H) / 2.0,
        reactor::FIELD_W / 2.0,
        (reactor::FIELD_H + reactor::FIELD_NAV_H) / 2.0,
    )
}

pub fn gen_random_pos_in_field(padding: f32) -> Vec2 {
    let mut rng = thread_rng();
    let rect = get_field_rect();
    Vec2::new(
        rng.gen_range(rect.min.x + padding..rect.max.x - padding),
        rng.gen_range(rect.min.y + padding..rect.max.y - padding),
    )
}

pub fn format_field_text(field: &str, value: u32) -> String {
    if field == "time" {
        format!("{:0>5}.{:0>2}", value / 100, value % 100)
    } else if field == "alpha_count" {
        format!("{:0>5}", value)
    } else if field == "score" {
        format!("{:0>8}", value)
    } else {
        format!("{}", value)
    }
}

pub fn build_reactor_field(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn((NodeBundle {
                    style: Style {
                        width: Val::Px(app::WINDOW_W),
                        height: Val::Px(app::WINDOW_H),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        border: UiRect::all(app::ui::px_p(0.5)),
                        ..default()
                    },
                    border_color: FIELD_COLOR.into(),
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn((NodeBundle {
                        style: Style {
                            flex_grow: 1.0,
                            ..default()
                        },
                        ..default()
                    },));
                    parent
                        .spawn((NodeBundle {
                            style: Style {
                                height: Val::Px(reactor::FIELD_NAV_H),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceBetween,
                                border: UiRect::top(app::ui::px_p(0.5)),
                                ..default()
                            },
                            border_color: FIELD_COLOR.into(),
                            ..default()
                        },))
                        .with_children(|parent| {
                            parent.spawn((NodeBundle {
                                style: Style {
                                    width: Val::Px(FIELD_TEXT_SIZE * 0.6),
                                    height: Val::Px(FIELD_TEXT_SIZE * 0.6),
                                    border: UiRect::all(app::ui::px_p(0.5)),
                                    margin: UiRect::left(Val::Px(FIELD_PADDING * 1.4)),
                                    ..default()
                                },
                                border_color: FIELD_COLOR.into(),
                                ..default()
                            },));
                            parent
                                .spawn((NodeBundle {
                                    style: Style {
                                        flex_grow: 1.0,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::SpaceBetween,
                                        ..default()
                                    },
                                    ..default()
                                },))
                                .with_children(|parent| {
                                    parent
                                        .spawn((NodeBundle {
                                            style: Style {
                                                width: Val::Px(reactor::FIELD_W * 0.25),
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::Start,
                                                ..default()
                                            },
                                            ..default()
                                        },))
                                        .with_children(|parent| {
                                            let icon =
                                                asset_server.load("images/icons/timer-fill.png");
                                            parent.spawn(ImageBundle {
                                                style: Style {
                                                    width: Val::Px(FIELD_TEXT_SIZE),
                                                    height: Val::Px(FIELD_TEXT_SIZE),
                                                    margin: UiRect::all(Val::Px(FIELD_PADDING)),
                                                    ..default()
                                                },
                                                image: UiImage::new(icon),
                                                ..default()
                                            });
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    format_field_text("time", 0),
                                                    TextStyle {
                                                        font: asset_server
                                                            .load(app::ui::FONT_DIGIT),
                                                        font_size: FIELD_TEXT_SIZE,
                                                        color: FIELD_TEXT_COLOR,
                                                        ..default()
                                                    },
                                                ),
                                                FieldTimer(0),
                                            ));
                                        });
                                    parent
                                        .spawn((NodeBundle {
                                            style: Style {
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::Center,
                                                ..default()
                                            },
                                            ..default()
                                        },))
                                        .with_children(|parent| {
                                            let icon = asset_server
                                                .load("images/icons/circles-three-fill.png");
                                            parent.spawn(ImageBundle {
                                                style: Style {
                                                    width: Val::Px(FIELD_TEXT_SIZE),
                                                    height: Val::Px(FIELD_TEXT_SIZE),
                                                    margin: UiRect::all(Val::Px(FIELD_PADDING)),
                                                    ..default()
                                                },
                                                image: UiImage::new(icon),
                                                ..default()
                                            });
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    format_field_text("alpha_count", 0),
                                                    TextStyle {
                                                        font: asset_server
                                                            .load(app::ui::FONT_DIGIT),
                                                        font_size: FIELD_TEXT_SIZE,
                                                        color: FIELD_TEXT_COLOR,
                                                        ..default()
                                                    },
                                                ),
                                                FieldAlphaCount(0),
                                            ));
                                        });
                                    parent
                                        .spawn((NodeBundle {
                                            style: Style {
                                                width: Val::Px(reactor::FIELD_W * 0.31),
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::Start,
                                                margin: UiRect::right(Val::Px(FIELD_PADDING)),
                                                ..default()
                                            },
                                            ..default()
                                        },))
                                        .with_children(|parent| {
                                            let icon =
                                                asset_server.load("images/icons/trophy-fill.png");
                                            parent.spawn(ImageBundle {
                                                style: Style {
                                                    width: Val::Px(FIELD_TEXT_SIZE),
                                                    height: Val::Px(FIELD_TEXT_SIZE),
                                                    margin: UiRect::all(Val::Px(FIELD_PADDING)),
                                                    ..default()
                                                },
                                                image: UiImage::new(icon),
                                                ..default()
                                            });
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    format_field_text("score", 0),
                                                    TextStyle {
                                                        font: asset_server
                                                            .load(app::ui::FONT_DIGIT),
                                                        font_size: FIELD_TEXT_SIZE,
                                                        color: FIELD_TEXT_COLOR,
                                                        ..default()
                                                    },
                                                ),
                                                FieldScore(0),
                                            ));
                                        });
                                });
                        });
                });
        })
        .id()
}
