use bevy::{app::AppExit, prelude::*};
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
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            let icon = asset_server.load("images/app/logo_120x.png");
                            parent.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(120.0),
                                    height: Val::Px(120.0),
                                    margin: UiRect::right(app::ui::px_p(8.0)),
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
                                        font_size: app::ui::FONT_SIZE * 3.0,
                                        color: app::ui::FG_COLOR,
                                        ..default()
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::bottom(app::ui::px_p(10.0)),
                                    ..default()
                                }),
                            );
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            app::ui::build_menu_entry(
                                parent,
                                &asset_server,
                                (
                                    ButtonAction::FirstRun,
                                    app::interaction::IaButton,
                                    Focusable::new().prioritized(),
                                ),
                                "Start",
                                "play-light",
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
                                    ButtonAction::MoveToPage(app::GameState::Settings),
                                    app::interaction::IaButton,
                                    Focusable::default(),
                                ),
                                "Variables",
                                "gear-light",
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
                        });
                });
        });
}

fn handle_menu_navigation(
    mut actions: Query<&mut ButtonAction>,
    mut events: EventReader<NavEvent>,
    mut game_state: ResMut<NextState<app::GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut settings: ResMut<Persistent<app::settings::Settings>>,
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
            ButtonAction::Quit => app_exit_events.send(AppExit),
        },
    );
}
