use bevy::prelude::*;
use bevy_persistent::prelude::*;
use bevy_ui_navigation::{prelude::*, NavRequestSystem};

use crate::{app, page};

pub struct PagePlugin;

impl Plugin for PagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(app::GameState::Help), page_setup)
            .add_systems(
                Update,
                handle_ui_navigation
                    .after(NavRequestSystem)
                    .run_if(in_state(app::GameState::Help)),
            )
            .add_systems(OnExit(app::GameState::Help), app::ui::despawn_ui::<OnPage>);
    }
}

#[derive(Component)]
struct OnPage;

#[derive(Component)]
pub struct HelpPanel(u8);

#[derive(Component)]
pub struct HelpDot(u8);

#[derive(Component)]
enum ButtonAction {
    BackToMainMenu,
    PrevHelp,
    NextHelp,
    Start,
}

fn page_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut settings: ResMut<Persistent<app::settings::Settings>>,
) {
    if settings.is_enabled("first") {
        settings
            .update(|settings| {
                settings.toggle("first");
            })
            .expect("failed to update first run in help");
    }
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
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                align_items: AlignItems::Start,
                                justify_content: JustifyContent::SpaceBetween,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            page::build_game_title(parent, &asset_server);
                            page::build_page_title(
                                parent,
                                &asset_server,
                                "Formula",
                                "question-light",
                            );
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                flex_grow: 1.0,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            app::ui::build_icon_btn(
                                parent,
                                &asset_server,
                                (
                                    ButtonAction::PrevHelp,
                                    app::interaction::IaButton,
                                    Focusable::default(),
                                ),
                                Style::default(),
                                "caret-double-left-light",
                            );
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Start,
                                        justify_content: JustifyContent::Center,
                                        margin: UiRect::all(app::ui::px_p(4.0)),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    build_help_dots(parent);
                                    build_help_panel(parent, &asset_server);
                                });
                            app::ui::build_icon_btn(
                                parent,
                                &asset_server,
                                (
                                    ButtonAction::NextHelp,
                                    app::interaction::IaButton,
                                    Focusable::new().prioritized(),
                                ),
                                Style::default(),
                                "caret-double-right-light",
                            );
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceBetween,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            app::ui::build_icon_btn(
                                parent,
                                &asset_server,
                                (
                                    ButtonAction::BackToMainMenu,
                                    app::interaction::IaButton,
                                    Focusable::default(),
                                ),
                                Style::default(),
                                "arrow-left-light",
                            );
                            app::ui::build_btn(
                                parent,
                                &asset_server,
                                (
                                    ButtonAction::Start,
                                    app::interaction::IaButton,
                                    Focusable::default(),
                                ),
                                Style {
                                    padding: UiRect::all(app::ui::px_p(3.0)),
                                    ..default()
                                },
                                Some("Start"),
                                Some("play-light"),
                            );
                        });
                });
        });
}

fn handle_ui_navigation(
    mut actions: Query<&mut ButtonAction>,
    mut events: EventReader<NavEvent>,
    mut game_state: ResMut<NextState<app::GameState>>,
    mut help_panel_query: Query<(&mut HelpPanel, &mut UiImage), With<HelpPanel>>,
    mut help_dot_query: Query<(&HelpDot, &mut BackgroundColor), With<HelpDot>>,
    asset_server: Res<AssetServer>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut actions,
        |mut action| match &mut *action {
            ButtonAction::BackToMainMenu => game_state.set(app::GameState::Menu),
            ButtonAction::PrevHelp => {
                let (mut help_panel, mut image) = help_panel_query.single_mut();
                let prev_help = (help_panel.0 + HELP_COUNT - 1) % HELP_COUNT;
                help_panel.0 = prev_help;
                let icon_path = format!("images/help/{:0>2}.png", prev_help);
                let icon = asset_server.load(icon_path);
                image.texture = icon;
                for (help_dot, mut bg_color) in help_dot_query.iter_mut() {
                    if help_dot.0 == prev_help {
                        *bg_color = app::ui::MUTE_COLOR.into();
                    } else {
                        *bg_color = app::ui::BG_COLOR.into();
                    }
                }
            }
            ButtonAction::NextHelp => {
                let (mut help_panel, mut image) = help_panel_query.single_mut();
                let next_help = (help_panel.0 + 1) % HELP_COUNT;
                help_panel.0 = next_help;
                let icon_path = format!("images/help/{:0>2}.png", next_help);
                let icon = asset_server.load(icon_path);
                image.texture = icon;
                for (help_dot, mut bg_color) in help_dot_query.iter_mut() {
                    if help_dot.0 == next_help {
                        *bg_color = app::ui::MUTE_COLOR.into();
                    } else {
                        *bg_color = app::ui::BG_COLOR.into();
                    }
                }
            }
            ButtonAction::Start => game_state.set(app::GameState::Game),
        },
    );
}

const HELP_PANEL_SIZE: f32 = 156.0;
const HELP_COUNT: u8 = 14;

fn build_help_panel(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn((NodeBundle {
            style: Style {
                width: app::ui::px_p(HELP_PANEL_SIZE),
                height: app::ui::px_p(HELP_PANEL_SIZE),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(app::ui::px_p(0.5)),
                ..default()
            },
            background_color: app::ui::BG_COLOR.into(),
            border_color: app::ui::MUTE_COLOR.into(),
            ..default()
        },))
        .with_children(|parent| {
            let icon = asset_server.load("images/help/00.png");
            parent.spawn((
                ImageBundle {
                    image: UiImage::new(icon),
                    ..default()
                },
                HelpPanel(0),
            ));
        });
}

const HELP_DOT_SIZE: f32 = 6.0;

fn build_help_dots(parent: &mut ChildBuilder) {
    parent
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            for i in 0..HELP_COUNT {
                let mut bg_color = app::ui::BG_COLOR.into();
                if i == 0 {
                    bg_color = app::ui::MUTE_COLOR.into();
                }
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: app::ui::px_p(HELP_DOT_SIZE),
                            height: app::ui::px_p(HELP_DOT_SIZE),
                            margin: UiRect::new(
                                app::ui::px_p(2.0),
                                app::ui::px_p(2.0),
                                app::ui::px_p(0.0),
                                app::ui::px_p(4.0),
                            ),
                            border: UiRect::all(app::ui::px_p(0.5)),
                            ..default()
                        },
                        background_color: bg_color,
                        border_color: app::ui::MUTE_COLOR.into(),
                        ..default()
                    },
                    HelpDot(i),
                ));
            }
        });
}
