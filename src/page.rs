use crate::app;
use bevy::prelude::*;

pub mod about;
pub mod game;
pub mod help;
pub mod menu;
pub mod settings;

const PAGE_PADDING: f32 = 4.0;
const PAGE_TITLE_RATIO: f32 = 1.2;
const SEP_W: f32 = 450.0;

pub fn build_page_layout() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(app::ui::px_p(PAGE_PADDING)),
            ..default()
        },
        ..default()
    }
}

pub fn build_game_title(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn(
        TextBundle::from_section(
            "Chain Reaction",
            TextStyle {
                font: asset_server.load(app::ui::FONT),
                font_size: app::ui::FONT_SIZE * 2.0,
                color: app::ui::FG_COLOR,
                ..default()
            },
        )
        .with_style(Style {
            margin: UiRect::left(app::ui::px_p(2.0)),
            ..default()
        }),
    );
}

pub fn build_page_title(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    text: &str,
    icon: &str,
) -> Entity {
    parent
        .spawn((NodeBundle {
            style: Style {
                height: Val::Auto,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect::top(app::ui::px_p(3.0)),
                padding: UiRect::all(app::ui::px_p(2.0)),
                border: UiRect::all(app::ui::px_p(0.5)),
                ..default()
            },
            background_color: app::ui::BG_COLOR.into(),
            border_color: app::ui::FG_COLOR.into(),
            ..default()
        },))
        .with_children(|parent| {
            let icon_path = format!("images/icons/{}.png", icon);
            let icon = asset_server.load(icon_path);
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(app::ui::ICON_SIZE * PAGE_TITLE_RATIO),
                    height: Val::Px(app::ui::ICON_SIZE * PAGE_TITLE_RATIO),
                    margin: UiRect::right(app::ui::px_p(4.0)),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
            parent.spawn(
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load(app::ui::FONT),
                        font_size: app::ui::FONT_SIZE * PAGE_TITLE_RATIO,
                        color: app::ui::FG_COLOR,
                    },
                )
                .with_style(Style {
                    margin: UiRect::right(app::ui::px_p(2.0)),
                    ..default()
                }),
            );
        })
        .id()
}

pub fn build_sep_title(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    text: &str,
    icon: &str,
) -> Entity {
    parent
        .spawn((NodeBundle {
            style: Style {
                width: Val::Px(SEP_W),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::vertical(app::ui::px_p(3.0)),
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px(SEP_W),
                    height: app::ui::px_p(1.0),
                    margin: UiRect::top(app::ui::px_p(3.0)),
                    ..default()
                },
                background_color: app::ui::SECONDARY_COLOR.into(),
                ..default()
            },));
            parent
                .spawn((NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(app::ui::px_p(3.0)),
                        ..default()
                    },
                    background_color: app::ui::BG_COLOR.into(),
                    ..default()
                },))
                .with_children(|parent| {
                    let icon_path = format!("images/icons/{}.png", icon);
                    let icon = asset_server.load(icon_path);
                    parent.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(app::ui::ICON_SIZE * PAGE_TITLE_RATIO),
                            height: Val::Px(app::ui::ICON_SIZE * PAGE_TITLE_RATIO),
                            margin: UiRect::right(app::ui::px_p(4.0)),
                            ..default()
                        },
                        image: UiImage::new(icon),
                        ..default()
                    });
                    parent.spawn(
                        TextBundle::from_section(
                            text,
                            TextStyle {
                                font: asset_server.load(app::ui::FONT),
                                font_size: app::ui::FONT_SIZE * PAGE_TITLE_RATIO,
                                color: app::ui::SECONDARY_COLOR,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::right(app::ui::px_p(2.0)),
                            ..default()
                        }),
                    );
                });
        })
        .id()
}