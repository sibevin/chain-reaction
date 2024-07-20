use super::*;
use crate::app::{theme, ui};
use crate::game::{achievement, anime_end};
use bevy::sprite::Anchor;
use bevy_persistent::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, *};
use std::time::Duration;

const FIELD_ACH_W: f32 = 380.0;
const FIELD_ACH_H: f32 = FIELD_BAR_H - FIELD_P * 2.0;
const ACH_ICON_SIZE: f32 = ui::FONT_SIZE * 2.0;
const ACH_ICON_BORDER_W: f32 = ACH_ICON_SIZE * 0.06;
const ACH_DESC_FS: f32 = ui::FONT_SIZE * 0.8;
const ACH_NAME_FS: f32 = ui::FONT_SIZE * 1.2;
const ACH_P: f32 = ui::FONT_SIZE * 0.3;
const ACH_ANIME_MOVING_DELTA: f32 = ui::FONT_SIZE;
const ACH_UI_P: f32 = ui::FONT_SIZE * 0.7;
const ACH_DOT_SIZE: f32 = ui::FONT_SIZE * 0.45;
const ACH_DOT_P: f32 = ACH_DOT_SIZE * 0.5;
const ACH_BAR_H: f32 = ACH_DOT_SIZE;
const ACH_BAR_W: f32 = FIELD_ACH_W - ACH_P * 3.5 - ACH_ICON_SIZE;

#[derive(Component)]
pub struct AchFieldUi(String);

pub enum AchPanel {
    Right,
    Left,
}

pub fn draw_ach_panels(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    game_status: &ResMut<GameStatus>,
    ach_info: &mut ResMut<achievement::AchievementInfo>,
    ach_store: &mut ResMut<Persistent<achievement::AchievementStore>>,
) {
    ach_info.reset(&ach_store);
    let codes = ach_info.running_codes();
    if let Some(code) = codes.get(0) {
        draw_ach_panel(
            parent,
            asset_server,
            game_status,
            AchPanel::Right,
            code,
            ach_store.is_pinned(code),
        );
    }
    if let Some(code) = codes.get(1) {
        draw_ach_panel(
            parent,
            asset_server,
            game_status,
            AchPanel::Left,
            code,
            ach_store.is_pinned(code),
        );
    }
    let next_code = ach_info.next_done_code();
    if let Some(code) = next_code {
        draw_ach_done_panel(parent, asset_server, &code);
    }
}

pub fn refresh_ach_panels(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    game_status: &ResMut<GameStatus>,
    ach_info: &mut ResMut<achievement::AchievementInfo>,
    ach_store: &mut ResMut<Persistent<achievement::AchievementStore>>,
) {
    ach_info.reset(&ach_store);
    let codes = ach_info.running_codes();
    if let Some(code) = codes.get(0) {
        draw_ach_panel(
            parent,
            asset_server,
            game_status,
            AchPanel::Right,
            code,
            ach_store.is_pinned(code),
        );
    }
    if let Some(code) = codes.get(1) {
        draw_ach_panel(
            parent,
            asset_server,
            game_status,
            AchPanel::Left,
            code,
            ach_store.is_pinned(code),
        );
    }
    let next_code = ach_info.next_done_code();
    if let Some(code) = next_code {
        draw_ach_done_panel(parent, asset_server, &code);
    }
}

pub fn draw_ach_panel(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    game_status: &ResMut<GameStatus>,
    panel: AchPanel,
    code: &str,
    is_pinned: bool,
) {
    let ach = fetch_ach_def(code);
    let rect = shapes::Rectangle {
        extents: Vec2::new(FIELD_ACH_W, FIELD_ACH_H),
        ..default()
    };
    let half_field_w = FIELD_W / 2.0;
    let half_field_h = FIELD_H / 2.0;
    let header_cy = half_field_h - FIELD_BAR_H / 2.0;
    let (start, end) = match panel {
        AchPanel::Right => {
            let end_pos = Vec2::new(-half_field_w + FIELD_P + FIELD_ACH_W / 2.0, header_cy);
            let start_pos = end_pos + Vec2::new(-ACH_ANIME_MOVING_DELTA, 0.0);
            (start_pos, end_pos)
        }
        AchPanel::Left => {
            let end_pos = Vec2::new(half_field_w - FIELD_P - FIELD_ACH_W / 2.0, header_cy);
            let start_pos = end_pos + Vec2::new(ACH_ANIME_MOVING_DELTA, 0.0);
            (start_pos, end_pos)
        }
    };
    let tween = Tween::new(
        EaseFunction::CubicOut,
        Duration::from_millis(500),
        TransformPositionLens {
            start: Vec3::new(start.x, start.y, 0.004),
            end: Vec3::new(end.x, end.y, 0.004),
        },
    );
    parent
        .spawn((SpatialBundle { ..default() }, Animator::new(tween)))
        .with_children(|parent| {
            let icon_pos = Vec2::new(-FIELD_ACH_W / 2.0 + ACH_ICON_SIZE / 2.0 + ACH_P, 0.0);
            let texture = if is_pinned {
                asset_server.load("images/icons/push-pin_2x.png")
            } else {
                asset_server.load("images/icons/crosshair_2x.png")
            };
            parent.spawn((SpriteBundle {
                texture,
                transform: Transform::from_xyz(icon_pos.x, icon_pos.y, 0.005),
                ..default()
            },));
            let icon_border = shapes::Rectangle {
                extents: Vec2::new(ACH_ICON_SIZE * 0.85, ACH_ICON_SIZE * 0.85),
                ..default()
            };
            let geo_builder = GeometryBuilder::new().add(&icon_border);
            parent.spawn((
                ShapeBundle {
                    path: geo_builder.build(),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(icon_pos.x, icon_pos.y, 0.005),
                        ..default()
                    },
                    ..default()
                },
                Stroke::new(theme::MUTE_COLOR, ACH_ICON_BORDER_W),
            ));
            let geo_builder = GeometryBuilder::new().add(&rect);
            parent.spawn((
                ShapeBundle {
                    path: geo_builder.build(),
                    ..default()
                },
                Stroke::new(FIELD_COLOR, FIELD_LINE_W),
                Fill::color(theme::BG_COLOR),
            ));
            let mut text_pos = Vec2::new(
                -FIELD_ACH_W / 2.0 + ACH_P * 2.0 + ACH_ICON_SIZE,
                FIELD_ACH_H / 2.0 - ACH_P * 1.5,
            );
            parent.spawn((Text2dBundle {
                text: Text::from_section(
                    ach.description(),
                    TextStyle {
                        font: asset_server.load(theme::FONT),
                        font_size: ACH_DESC_FS,
                        color: FIELD_TEXT_COLOR,
                    },
                ),
                text_anchor: Anchor::TopLeft,
                transform: Transform::from_xyz(text_pos.x, text_pos.y, 0.005),
                ..default()
            },));
            text_pos.y = text_pos.y - ACH_DESC_FS - ACH_UI_P;
            let (_, target, _) = ach.check_done(game_status);
            parent
                .spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(text_pos.x, text_pos.y, 0.005),
                        ..default()
                    },
                    AchFieldUi(String::from(ach.code())),
                ))
                .with_children(|parent| match ach.progress_ui() {
                    AchievementProgressUi::Dots => {
                        for i in 1..=target {
                            let pos = Vec2::new(
                                ACH_DOT_SIZE / 2.0 + (i - 1) as f32 * (ACH_DOT_SIZE + ACH_DOT_P),
                                0.0,
                            );
                            let dot = shapes::Rectangle {
                                extents: Vec2::new(ACH_DOT_SIZE, ACH_DOT_SIZE),
                                ..default()
                            };
                            let geo_builder = GeometryBuilder::new().add(&dot);
                            parent.spawn((
                                ShapeBundle {
                                    path: geo_builder.build(),
                                    spatial: SpatialBundle {
                                        transform: Transform::from_xyz(pos.x, pos.y, 0.005),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Fill::color(theme::MUTE_COLOR),
                            ));
                        }
                    }
                    AchievementProgressUi::Bar => {
                        let pos = Vec2::new(ACH_BAR_W / 2.0, 0.0);
                        let bar = shapes::Rectangle {
                            extents: Vec2::new(ACH_BAR_W, ACH_BAR_H),
                            ..default()
                        };
                        let geo_builder = GeometryBuilder::new().add(&bar);
                        parent.spawn((
                            ShapeBundle {
                                path: geo_builder.build(),
                                spatial: SpatialBundle {
                                    transform: Transform::from_xyz(pos.x, pos.y, 0.005),
                                    ..default()
                                },
                                ..default()
                            },
                            Fill::color(theme::MUTE_COLOR),
                        ));
                    }
                });
        });
}

pub fn refresh_ach_panel(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    game_status: &ResMut<GameStatus>,
    panel: AchPanel,
    code: &str,
    is_pinned: bool,
) {
    let ach = fetch_ach_def(code);
    let rect = shapes::Rectangle {
        extents: Vec2::new(FIELD_ACH_W, FIELD_ACH_H),
        ..default()
    };
    let half_field_w = FIELD_W / 2.0;
    let half_field_h = FIELD_H / 2.0;
    let header_cy = half_field_h - FIELD_BAR_H / 2.0;
    let (start, end) = match panel {
        AchPanel::Right => {
            let end_pos = Vec2::new(-half_field_w + FIELD_P + FIELD_ACH_W / 2.0, header_cy);
            let start_pos = end_pos + Vec2::new(-ACH_ANIME_MOVING_DELTA, 0.0);
            (start_pos, end_pos)
        }
        AchPanel::Left => {
            let end_pos = Vec2::new(half_field_w - FIELD_P - FIELD_ACH_W / 2.0, header_cy);
            let start_pos = end_pos + Vec2::new(ACH_ANIME_MOVING_DELTA, 0.0);
            (start_pos, end_pos)
        }
    };
    let tween = Tween::new(
        EaseFunction::CubicOut,
        Duration::from_millis(500),
        TransformPositionLens {
            start: Vec3::new(start.x, start.y, 0.004),
            end: Vec3::new(end.x, end.y, 0.004),
        },
    );
    parent
        .spawn((SpatialBundle { ..default() }, Animator::new(tween)))
        .with_children(|parent| {
            let icon_pos = Vec2::new(-FIELD_ACH_W / 2.0 + ACH_ICON_SIZE / 2.0 + ACH_P, 0.0);
            let texture = if is_pinned {
                asset_server.load("images/icons/push-pin_2x.png")
            } else {
                asset_server.load("images/icons/crosshair_2x.png")
            };
            parent.spawn((SpriteBundle {
                texture,
                transform: Transform::from_xyz(icon_pos.x, icon_pos.y, 0.005),
                ..default()
            },));
            let icon_border = shapes::Rectangle {
                extents: Vec2::new(ACH_ICON_SIZE * 0.85, ACH_ICON_SIZE * 0.85),
                ..default()
            };
            let geo_builder = GeometryBuilder::new().add(&icon_border);
            parent.spawn((
                ShapeBundle {
                    path: geo_builder.build(),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(icon_pos.x, icon_pos.y, 0.005),
                        ..default()
                    },
                    ..default()
                },
                Stroke::new(theme::MUTE_COLOR, ACH_ICON_BORDER_W),
            ));
            let geo_builder = GeometryBuilder::new().add(&rect);
            parent.spawn((
                ShapeBundle {
                    path: geo_builder.build(),
                    ..default()
                },
                Stroke::new(FIELD_COLOR, FIELD_LINE_W),
                Fill::color(theme::BG_COLOR),
            ));
            let mut text_pos = Vec2::new(
                -FIELD_ACH_W / 2.0 + ACH_P * 2.0 + ACH_ICON_SIZE,
                FIELD_ACH_H / 2.0 - ACH_P * 1.5,
            );
            parent.spawn((Text2dBundle {
                text: Text::from_section(
                    ach.description(),
                    TextStyle {
                        font: asset_server.load(theme::FONT),
                        font_size: ACH_DESC_FS,
                        color: FIELD_TEXT_COLOR,
                    },
                ),
                text_anchor: Anchor::TopLeft,
                transform: Transform::from_xyz(text_pos.x, text_pos.y, 0.005),
                ..default()
            },));
            text_pos.y = text_pos.y - ACH_DESC_FS - ACH_UI_P;
            let (current, target, _) = ach.check_done(game_status);
            parent
                .spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(text_pos.x, text_pos.y, 0.005),
                        ..default()
                    },
                    AchFieldUi(String::from(ach.code())),
                ))
                .with_children(|parent| match ach.progress_ui() {
                    AchievementProgressUi::Dots => {
                        for i in 1..=target {
                            let pos = Vec2::new(
                                ACH_DOT_SIZE / 2.0 + (i - 1) as f32 * (ACH_DOT_SIZE + ACH_DOT_P),
                                0.0,
                            );
                            let dot = shapes::Rectangle {
                                extents: Vec2::new(ACH_DOT_SIZE, ACH_DOT_SIZE),
                                ..default()
                            };
                            let color = if i <= current {
                                ach.color()
                            } else {
                                theme::MUTE_COLOR
                            };
                            let geo_builder = GeometryBuilder::new().add(&dot);
                            parent.spawn((
                                ShapeBundle {
                                    path: geo_builder.build(),
                                    spatial: SpatialBundle {
                                        transform: Transform::from_xyz(pos.x, pos.y, 0.005),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Fill::color(color),
                            ));
                        }
                    }
                    AchievementProgressUi::Bar => {
                        let pos = Vec2::new(ACH_BAR_W / 2.0, 0.0);
                        let bar = shapes::Rectangle {
                            extents: Vec2::new(ACH_BAR_W, ACH_BAR_H),
                            ..default()
                        };
                        let geo_builder = GeometryBuilder::new().add(&bar);
                        parent.spawn((
                            ShapeBundle {
                                path: geo_builder.build(),
                                spatial: SpatialBundle {
                                    transform: Transform::from_xyz(pos.x, pos.y, 0.005),
                                    ..default()
                                },
                                ..default()
                            },
                            Fill::color(theme::MUTE_COLOR),
                        ));
                    }
                });
        });
}

pub fn draw_ach_done_panel(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, code: &str) {
    let ach = fetch_ach_def(code);
    let rect = shapes::Rectangle {
        extents: Vec2::new(FIELD_ACH_W, FIELD_ACH_H),
        ..default()
    };
    let half_field_h = FIELD_H / 2.0;
    let header_cy = half_field_h - FIELD_BAR_H / 2.0;
    let end = Vec2::new(0.0, header_cy);
    let start = end + Vec2::new(0.0, ACH_ANIME_MOVING_DELTA);
    let tween = Tween::new(
        EaseFunction::CubicOut,
        Duration::from_millis(500),
        TransformPositionLens {
            start: Vec3::new(start.x, start.y, 0.004),
            end: Vec3::new(end.x, end.y, 0.004),
        },
    )
    .then(Delay::new(Duration::from_secs(3)).with_completed_event(anime_end::ACH_DONE_ANIME_END));
    parent
        .spawn((SpatialBundle { ..default() }, Animator::new(tween)))
        .with_children(|parent| {
            let icon_pos = Vec2::new(-FIELD_ACH_W / 2.0 + ACH_ICON_SIZE / 2.0 + ACH_P, 0.0);
            parent.spawn((SpriteBundle {
                texture: asset_server.load(ach.icon_path()),
                transform: Transform::from_xyz(icon_pos.x, icon_pos.y, 0.005),
                ..default()
            },));
            let geo_builder = GeometryBuilder::new().add(&rect);
            parent.spawn((
                ShapeBundle {
                    path: geo_builder.build(),
                    ..default()
                },
                Stroke::new(ach.color(), FIELD_LINE_W),
                Fill::color(theme::BG_COLOR),
            ));
            let mut text_pos = Vec2::new(
                -FIELD_ACH_W / 2.0 + ACH_P * 2.0 + ACH_ICON_SIZE,
                FIELD_ACH_H / 2.0 - ACH_P,
            );
            parent.spawn((Text2dBundle {
                text: Text::from_section(
                    ach.description(),
                    TextStyle {
                        font: asset_server.load(theme::FONT),
                        font_size: ACH_DESC_FS,
                        color: FIELD_TEXT_COLOR,
                    },
                ),
                text_anchor: Anchor::TopLeft,
                transform: Transform::from_xyz(text_pos.x, text_pos.y, 0.005),
                ..default()
            },));
            text_pos.y = text_pos.y - ACH_DESC_FS;
            parent.spawn((Text2dBundle {
                text: Text::from_section(
                    ach.name(),
                    TextStyle {
                        font: asset_server.load(theme::FONT),
                        font_size: ACH_NAME_FS,
                        color: ach.color(),
                    },
                ),
                text_anchor: Anchor::TopLeft,
                transform: Transform::from_xyz(text_pos.x, text_pos.y, 0.005),
                ..default()
            },));
        });
}
