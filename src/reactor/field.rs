use crate::{app, reactor};
use bevy::prelude::*;

const FIELD_TEXT_SIZE: f32 = reactor::FIELD_NAV_H * 0.5;
const FIELD_PADDING: f32 = (reactor::FIELD_NAV_H - FIELD_TEXT_SIZE) / 2.0;

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
                    border_color: reactor::FIELD_COLOR.into(),
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
                            border_color: reactor::FIELD_COLOR.into(),
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
                                border_color: reactor::FIELD_COLOR.into(),
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
                                                    reactor::timer::format_timer_text(0),
                                                    TextStyle {
                                                        font: asset_server
                                                            .load(app::ui::FONT_DIGIT),
                                                        font_size: FIELD_TEXT_SIZE,
                                                        color: reactor::FIELD_COLOR,
                                                        ..default()
                                                    },
                                                ),
                                                reactor::FieldTimer(0),
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
                                                    "00000",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load(app::ui::FONT_DIGIT),
                                                        font_size: FIELD_TEXT_SIZE,
                                                        color: reactor::FIELD_COLOR,
                                                        ..default()
                                                    },
                                                ),
                                                reactor::FieldAlphaCount(0),
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
                                                    "00000000000",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load(app::ui::FONT_DIGIT),
                                                        font_size: FIELD_TEXT_SIZE,
                                                        color: reactor::FIELD_COLOR,
                                                        ..default()
                                                    },
                                                ),
                                                reactor::FieldScore(0),
                                            ));
                                        });
                                });
                        });
                });
        })
        .id()
}
