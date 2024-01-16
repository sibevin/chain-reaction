#[cfg(not(target_arch = "wasm32"))]
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_persistent::prelude::*;
use bevy_ui_navigation::{prelude::*, NavRequestSystem};

use crate::{app, page};

pub struct PagePlugin;

impl Plugin for PagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(app::GameState::Menu), page_setup)
            .add_systems(
                Update,
                handle_menu_navigation
                    .after(NavRequestSystem)
                    .run_if(in_state(app::GameState::Menu)),
            )
            .add_systems(OnExit(app::GameState::Menu), app::ui::despawn_ui::<OnPage>);
    }
}

#[derive(Component)]
struct OnPage;

#[derive(Component)]
enum ButtonAction {
    FirstRun,
    MoveToPage(app::GameState),
    #[cfg(not(target_arch = "wasm32"))]
    Quit,
}

fn page_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((page::build_page_layout(), OnPage))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::bottom(app::ui::px_p(30.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                margin: UiRect::bottom(app::ui::px_p(12.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            let icon = asset_server.load("images/app/logo.png");
                            parent.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(80.0),
                                    height: Val::Px(80.0),
                                    margin: UiRect::right(app::ui::px_p(6.0)),
                                    ..default()
                                },
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(
                                TextBundle::from_section(
                                    "Chain Reaction",
                                    TextStyle {
                                        font: asset_server.load(app::ui::FONT),
                                        font_size: app::ui::FONT_SIZE * 2.8,
                                        color: app::ui::FG_COLOR,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::bottom(app::ui::px_p(4.0)),
                                    ..default()
                                }),
                            );
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(
                                        app::ui::MENU_ENTRY_W * 2.0 + app::ui::MENU_ENTRY_PADDING,
                                    ),
                                    justify_content: JustifyContent::SpaceBetween,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(app::ui::px_p(4.0)),
                                    margin: UiRect::bottom(Val::Px(app::ui::MENU_ENTRY_PADDING)),
                                    ..default()
                                },
                                background_color: app::ui::BG_COLOR.into(),
                                ..default()
                            },
                            ButtonAction::FirstRun,
                            app::interaction::IaButton,
                            Focusable::new().prioritized(),
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("images/icons/play-light.png");
                            parent.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(app::ui::ICON_SIZE * 1.6),
                                    height: Val::Px(app::ui::ICON_SIZE * 1.6),
                                    margin: UiRect::right(app::ui::px_p(3.0)),
                                    ..default()
                                },
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(
                                TextBundle::from_section(
                                    "Start",
                                    TextStyle {
                                        font: asset_server.load(app::ui::FONT),
                                        font_size: app::ui::FONT_SIZE * 1.6,
                                        color: app::ui::FG_COLOR,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::right(app::ui::px_p(2.0)),
                                    ..default()
                                }),
                            );
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Start,
                                column_gap: Val::Px(app::ui::MENU_ENTRY_PADDING),
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
                                        row_gap: Val::Px(app::ui::MENU_ENTRY_PADDING),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    app::ui::build_menu_entry(
                                        parent,
                                        &asset_server,
                                        (
                                            ButtonAction::MoveToPage(app::GameState::Leaderboard),
                                            app::interaction::IaButton,
                                            Focusable::default(),
                                        ),
                                        "Report",
                                        "list-numbers",
                                    );
                                    app::ui::build_menu_entry(
                                        parent,
                                        &asset_server,
                                        (
                                            ButtonAction::MoveToPage(app::GameState::Help),
                                            app::interaction::IaButton,
                                            Focusable::default(),
                                        ),
                                        "Formula",
                                        "question-light",
                                    );
                                    app::ui::build_menu_entry(
                                        parent,
                                        &asset_server,
                                        (
                                            ButtonAction::MoveToPage(app::GameState::About),
                                            app::interaction::IaButton,
                                            Focusable::default(),
                                        ),
                                        "References",
                                        "star-light",
                                    );
                                });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        row_gap: Val::Px(app::ui::MENU_ENTRY_PADDING),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    app::ui::build_menu_entry(
                                        parent,
                                        &asset_server,
                                        (
                                            ButtonAction::MoveToPage(app::GameState::Achievement),
                                            app::interaction::IaButton,
                                            Focusable::default(),
                                        ),
                                        "Marks",
                                        "crosshair",
                                    );
                                    app::ui::build_menu_entry(
                                        parent,
                                        &asset_server,
                                        (
                                            ButtonAction::MoveToPage(app::GameState::Settings),
                                            app::interaction::IaButton,
                                            Focusable::default(),
                                        ),
                                        "Variables",
                                        "gear-light",
                                    );
                                    #[cfg(not(target_arch = "wasm32"))]
                                    {
                                        app::ui::build_menu_entry(
                                            parent,
                                            &asset_server,
                                            (
                                                ButtonAction::Quit,
                                                app::interaction::IaButton,
                                                Focusable::default(),
                                            ),
                                            "Quit",
                                            "sign-out-light",
                                        );
                                    }
                                });
                        });
                });
            app::ui::build_icon_btn(
                parent,
                &asset_server,
                (
                    ButtonAction::MoveToPage(app::GameState::Auto),
                    app::interaction::IaButton,
                    Focusable::default(),
                ),
                Style {
                    position_type: PositionType::Absolute,
                    bottom: app::ui::px_p(page::PAGE_PADDING),
                    left: app::ui::px_p(page::PAGE_PADDING),
                    ..default()
                },
                "monitor",
            );
        });
}

fn handle_menu_navigation(
    mut actions: Query<&mut ButtonAction>,
    mut events: EventReader<NavEvent>,
    mut game_state: ResMut<NextState<app::GameState>>,
    mut settings: ResMut<Persistent<app::settings::Settings>>,
    #[cfg(not(target_arch = "wasm32"))] mut app_exit_events: EventWriter<AppExit>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut actions,
        |mut action| match &mut *action {
            ButtonAction::FirstRun => {
                if settings.is_enabled("first") {
                    settings.toggle("first");
                    game_state.set(app::GameState::Help)
                } else {
                    game_state.set(app::GameState::Game)
                }
            }
            ButtonAction::MoveToPage(state) => game_state.set(*state),
            #[cfg(not(target_arch = "wasm32"))]
            ButtonAction::Quit => app_exit_events.send(AppExit),
        },
    );
}
