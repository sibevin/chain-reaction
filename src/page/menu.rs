use bevy::{app::AppExit, prelude::*};

use crate::{app, page};

pub struct PagePlugin;

impl Plugin for PagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(app::GameState::Menu), page_setup)
            .add_systems(Update, page_action.run_if(in_state(app::GameState::Menu)))
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
                                ButtonAction::FirstRun,
                                "Start",
                                "play-light",
                            );
                            app::ui::build_menu_entry(
                                parent,
                                &asset_server,
                                ButtonAction::MoveToPage(app::GameState::Help),
                                "Formula",
                                "question-light",
                            );
                            app::ui::build_menu_entry(
                                parent,
                                &asset_server,
                                ButtonAction::MoveToPage(app::GameState::Settings),
                                "Variables",
                                "gear-light",
                            );
                            app::ui::build_menu_entry(
                                parent,
                                &asset_server,
                                ButtonAction::MoveToPage(app::GameState::About),
                                "References",
                                "star-light",
                            );
                            app::ui::build_menu_entry(
                                parent,
                                &asset_server,
                                ButtonAction::Quit,
                                "Quit",
                                "sign-out-light",
                            );
                        });
                });
        });
}

fn page_action(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<NextState<app::GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut settings: ResMut<app::settings::Settings>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
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
            }
        }
    }
}
