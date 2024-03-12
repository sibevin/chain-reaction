use crate::{app::theme, app::ui, book::page::*};
use bevy_ui_navigation::prelude::*;

pub mod audio;
pub mod control;
pub mod display;

#[derive(Component, Debug)]
pub enum ButtonAction {
    MoveToPage(PageState),
    AppUiNav,
    PlaySe,
}

pub fn build_settings_nav_bar(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    page: PageState,
) -> Entity {
    parent
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                align_items: AlignItems::End,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            ui::build_icon_btn(
                parent,
                &asset_server,
                (
                    ButtonAction::MoveToPage(PageState::Menu),
                    app::interaction::IaButton,
                    Focusable::default(),
                    app::interaction::IaDefaultFocus,
                ),
                Style::default(),
                "arrow-left-light_1.5x",
            );
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_grow: 1.0,
                        align_items: AlignItems::End,
                        justify_content: JustifyContent::Center,
                        row_gap: ui::px_p(2.0),
                        margin: UiRect::right(ui::px_p(12.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::End,
                                row_gap: ui::px_p(1.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            if page == PageState::SettingsAudio {
                                build_current_tab(
                                    parent,
                                    &asset_server,
                                    "speaker-simple-high-light_1.5x",
                                );
                            } else {
                                ui::build_icon_btn(
                                    parent,
                                    &asset_server,
                                    (
                                        ButtonAction::MoveToPage(PageState::SettingsAudio),
                                        app::interaction::IaButton,
                                        Focusable::default(),
                                    ),
                                    Style::default(),
                                    "speaker-simple-high-light_1.5x",
                                );
                            }
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::End,
                                row_gap: ui::px_p(1.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            if page == PageState::SettingsDisplay {
                                build_current_tab(parent, &asset_server, "monitor-light_1.5x");
                            } else {
                                ui::build_icon_btn(
                                    parent,
                                    &asset_server,
                                    (
                                        ButtonAction::MoveToPage(PageState::SettingsDisplay),
                                        app::interaction::IaButton,
                                        Focusable::default(),
                                    ),
                                    Style::default(),
                                    "monitor-light_1.5x",
                                );
                            }
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::End,
                                row_gap: ui::px_p(1.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            if page == PageState::SettingsControl {
                                build_current_tab(
                                    parent,
                                    &asset_server,
                                    "game-controller-light_1.5x",
                                );
                            } else {
                                ui::build_icon_btn(
                                    parent,
                                    &asset_server,
                                    (
                                        ButtonAction::MoveToPage(PageState::SettingsControl),
                                        app::interaction::IaButton,
                                        Focusable::default(),
                                    ),
                                    Style::default(),
                                    "game-controller-light_1.5x",
                                );
                            }
                        });
                });
        })
        .id()
}

fn build_current_tab(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, tab_icon: &str) {
    let icon = asset_server.load("images/icons/circle-fill_16.png");
    parent.spawn(ImageBundle {
        style: Style {
            width: Val::Px(16.0),
            height: Val::Px(16.0),
            ..default()
        },
        image: UiImage::new(icon),
        ..default()
    });
    parent
        .spawn((NodeBundle {
            style: Style {
                width: Val::Auto,
                height: Val::Auto,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::new(
                    ui::px_p(ui::BTN_PADDING * 0.6),
                    ui::px_p(ui::BTN_PADDING * 0.6),
                    ui::px_p(ui::BTN_PADDING * 0.3),
                    ui::px_p(ui::BTN_PADDING * 0.6),
                ),
                ..default()
            },
            background_color: theme::BTN_BG.into(),
            ..default()
        },))
        .with_children(|parent| {
            let icon_path = format!("images/icons/{}.png", tab_icon);
            let icon = asset_server.load(icon_path);
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(ui::ICON_SIZE * 1.5),
                    height: Val::Px(ui::ICON_SIZE * 1.5),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
        });
}
