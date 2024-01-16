use crate::{
    app::{self, ui::BG_COLOR},
    page, reactor,
};
use bevy::prelude::*;
use bevy_ui_navigation::{prelude::*, NavRequestSystem};

pub struct PagePlugin;

impl Plugin for PagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(app::GameState::Dev), page_setup)
            .add_systems(
                Update,
                handle_ui_navigation
                    .after(NavRequestSystem)
                    .run_if(in_state(app::GameState::Dev)),
            )
            .add_systems(OnExit(app::GameState::Dev), app::ui::despawn_ui::<OnPage>);
    }
}

#[derive(Component)]
struct OnPage;

#[derive(Component)]
enum ButtonAction {
    MoveToPage(app::GameState),
}

#[derive(Component)]
struct LeaderboardList(String);

#[derive(Component)]
struct ScreenshotPanel;

#[derive(Component)]
struct ScreenshotImage;

const COLORS: [Color; 9] = [
    app::ui::FG_COLOR,
    app::ui::BG_COLOR,
    app::ui::SECONDARY_COLOR,
    app::ui::MUTE_COLOR,
    reactor::particle::alpha::COLOR,
    reactor::particle::control::COLOR,
    reactor::particle::hyper::COLOR,
    reactor::particle::trigger::COLOR,
    reactor::particle::uou::COLOR,
];

fn page_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((page::build_page_layout(), OnPage))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    page::build_game_title(parent, &asset_server);
                    page::build_page_title(parent, &asset_server, "Dev", "wrench");
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            page::build_sep_title(parent, &asset_server, "Font", "text-aa-fill");
                            parent.spawn(
                                TextBundle::from_section(
                                    "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789,.-",
                                    TextStyle {
                                        font: asset_server.load(app::ui::FONT),
                                        font_size: app::ui::FONT_SIZE,
                                        color: app::ui::FG_COLOR,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::vertical(app::ui::px_p(2.0)),
                                    ..default()
                                }),
                            );
                            parent.spawn(
                                TextBundle::from_section(
                                    "abcdefghijklmnopqrstuvwxyzα!@#$%^&*()+=",
                                    TextStyle {
                                        font: asset_server.load(app::ui::FONT),
                                        font_size: app::ui::FONT_SIZE,
                                        color: app::ui::FG_COLOR,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::vertical(app::ui::px_p(2.0)),
                                    ..default()
                                }),
                            );
                            parent.spawn(
                                TextBundle::from_section(
                                    "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789,.-",
                                    TextStyle {
                                        font: asset_server.load(app::ui::FONT_DIGIT),
                                        font_size: app::ui::FONT_SIZE,
                                        color: app::ui::FG_COLOR,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::vertical(app::ui::px_p(2.0)),
                                    ..default()
                                }),
                            );
                            parent.spawn(
                                TextBundle::from_section(
                                    "abcdefghijklmnopqrstuvwxyzα!@#$%^&*()+=",
                                    TextStyle {
                                        font: asset_server.load(app::ui::FONT_DIGIT),
                                        font_size: app::ui::FONT_SIZE,
                                        color: app::ui::FG_COLOR,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::vertical(app::ui::px_p(2.0)),
                                    ..default()
                                }),
                            );
                            parent.spawn(
                                TextBundle::from_section(
                                    "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789,.-",
                                    TextStyle {
                                        font: asset_server.load(app::ui::FONT_HW),
                                        font_size: app::ui::FONT_SIZE,
                                        color: app::ui::FG_COLOR,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::vertical(app::ui::px_p(2.0)),
                                    ..default()
                                }),
                            );
                            parent.spawn(
                                TextBundle::from_section(
                                    "abcdefghijklmnopqrstuvwxyzα!@#$%^&*()+=",
                                    TextStyle {
                                        font: asset_server.load(app::ui::FONT_HW),
                                        font_size: app::ui::FONT_SIZE,
                                        color: app::ui::FG_COLOR,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::vertical(app::ui::px_p(2.0)),
                                    ..default()
                                }),
                            );
                            page::build_sep_title(parent, &asset_server, "Color", "palette-fill");
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    background_color: app::ui::BG_COLOR.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    for color in COLORS {
                                        parent
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    margin: UiRect::all(app::ui::px_p(2.0)),
                                                    padding: UiRect::all(app::ui::px_p(2.0)),
                                                    border: UiRect::all(app::ui::px_p(0.5)),
                                                    ..default()
                                                },
                                                background_color: BG_COLOR.into(),
                                                border_color: app::ui::FG_COLOR.into(),
                                                ..default()
                                            })
                                            .with_children(|parent| {
                                                parent.spawn(NodeBundle {
                                                    style: Style {
                                                        width: Val::Px(app::ui::ICON_SIZE * 1.5),
                                                        height: Val::Px(app::ui::ICON_SIZE * 1.5),
                                                        ..default()
                                                    },
                                                    background_color: color.into(),
                                                    ..default()
                                                });
                                            });
                                    }
                                });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    background_color: app::ui::BG_COLOR.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    for ach_def in app::achievement::ACHIEVEMENTS {
                                        parent
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    margin: UiRect::all(app::ui::px_p(2.0)),
                                                    padding: UiRect::all(app::ui::px_p(2.0)),
                                                    border: UiRect::all(app::ui::px_p(0.5)),
                                                    ..default()
                                                },
                                                background_color: BG_COLOR.into(),
                                                border_color: app::ui::FG_COLOR.into(),
                                                ..default()
                                            })
                                            .with_children(|parent| {
                                                parent.spawn(NodeBundle {
                                                    style: Style {
                                                        width: Val::Px(app::ui::ICON_SIZE * 1.5),
                                                        height: Val::Px(app::ui::ICON_SIZE * 1.5),
                                                        ..default()
                                                    },
                                                    background_color: ach_def.color().into(),
                                                    ..default()
                                                });
                                            });
                                    }
                                });
                        });
                });
            app::ui::build_icon_btn(
                parent,
                &asset_server,
                (
                    ButtonAction::MoveToPage(app::GameState::Menu),
                    app::interaction::IaButton,
                    Focusable::new().prioritized(),
                ),
                Style {
                    position_type: PositionType::Absolute,
                    bottom: app::ui::px_p(page::PAGE_PADDING),
                    left: app::ui::px_p(page::PAGE_PADDING),
                    ..default()
                },
                "arrow-left-light",
            );
        });
}

fn handle_ui_navigation(
    mut actions: Query<&mut ButtonAction>,
    mut events: EventReader<NavEvent>,
    mut game_state: ResMut<NextState<app::GameState>>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut actions,
        |mut action| match &mut *action {
            ButtonAction::MoveToPage(state) => game_state.set(*state),
        },
    );
}
