use bevy::prelude::*;

use crate::{app, page};

pub struct PagePlugin;

impl Plugin for PagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(app::GameState::Help), page_setup)
            .add_systems(Update, page_action.run_if(in_state(app::GameState::Help)))
            .add_systems(OnExit(app::GameState::Help), app::ui::despawn_ui::<OnPage>);
    }
}

#[derive(Component)]
struct OnPage;

#[derive(Component)]
struct HelpPanel;

#[derive(Component)]
struct HelpDot(u8);

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
    mut settings: ResMut<app::settings::Settings>,
) {
    if settings.is_enabled("first") {
        settings.toggle("first");
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
                                ButtonAction::PrevHelp,
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
                                    build_help_panel(parent);
                                    build_help_dots(parent);
                                });
                            app::ui::build_icon_btn(
                                parent,
                                &asset_server,
                                ButtonAction::NextHelp,
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
                                ButtonAction::BackToMainMenu,
                                Style::default(),
                                "arrow-left-light",
                            );
                            app::ui::build_btn(
                                parent,
                                &asset_server,
                                ButtonAction::Start,
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

fn page_action(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<NextState<app::GameState>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                ButtonAction::BackToMainMenu => game_state.set(app::GameState::Menu),
                ButtonAction::PrevHelp => (),
                ButtonAction::NextHelp => (),
                ButtonAction::Start => game_state.set(app::GameState::Game),
            }
        }
    }
}

const HELP_PANEL_SIZE: f32 = 150.0;
const MTP_BALL_SIZE: f32 = 5.0;
const MTP_BALL_POS: f32 = (HELP_PANEL_SIZE - MTP_BALL_SIZE) / 2.0;
const U_COLOR: Color = Color::rgb(1.0, 0.84, 0.2);

fn build_help_panel(parent: &mut ChildBuilder) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    width: app::ui::px_p(HELP_PANEL_SIZE),
                    height: app::ui::px_p(HELP_PANEL_SIZE),
                    border: UiRect::all(app::ui::px_p(0.5)),
                    ..default()
                },
                background_color: app::ui::BG_COLOR.into(),
                border_color: app::ui::MUTE_COLOR.into(),
                ..default()
            },
            HelpPanel,
        ))
        .with_children(|parent| {
            parent.spawn((NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: app::ui::px_p(MTP_BALL_SIZE),
                    height: app::ui::px_p(MTP_BALL_SIZE),
                    top: app::ui::px_p(MTP_BALL_POS),
                    left: app::ui::px_p(MTP_BALL_POS),
                    ..default()
                },
                background_color: U_COLOR.into(),
                ..default()
            },));
        });
}

const HELP_DOT_SIZE: f32 = 6.0;

fn build_help_dots(parent: &mut ChildBuilder) {
    parent
        .spawn((NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: app::ui::px_p(HELP_DOT_SIZE),
                        height: app::ui::px_p(HELP_DOT_SIZE),
                        margin: UiRect::new(
                            app::ui::px_p(0.0),
                            app::ui::px_p(4.0),
                            app::ui::px_p(4.0),
                            app::ui::px_p(0.0),
                        ),
                        border: UiRect::all(app::ui::px_p(0.5)),
                        ..default()
                    },
                    background_color: app::ui::BG_COLOR.into(),
                    border_color: app::ui::MUTE_COLOR.into(),
                    ..default()
                },
                HelpDot(0),
            ));
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: app::ui::px_p(HELP_DOT_SIZE),
                        height: app::ui::px_p(HELP_DOT_SIZE),
                        margin: UiRect::new(
                            app::ui::px_p(0.0),
                            app::ui::px_p(4.0),
                            app::ui::px_p(4.0),
                            app::ui::px_p(0.0),
                        ),
                        border: UiRect::all(app::ui::px_p(0.5)),
                        ..default()
                    },
                    background_color: app::ui::MUTE_COLOR.into(),
                    border_color: app::ui::MUTE_COLOR.into(),
                    ..default()
                },
                HelpDot(0),
            ));
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: app::ui::px_p(HELP_DOT_SIZE),
                        height: app::ui::px_p(HELP_DOT_SIZE),
                        margin: UiRect::new(
                            app::ui::px_p(0.0),
                            app::ui::px_p(4.0),
                            app::ui::px_p(4.0),
                            app::ui::px_p(0.0),
                        ),
                        border: UiRect::all(app::ui::px_p(0.5)),
                        ..default()
                    },
                    background_color: app::ui::BG_COLOR.into(),
                    border_color: app::ui::MUTE_COLOR.into(),
                    ..default()
                },
                HelpDot(0),
            ));
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: app::ui::px_p(HELP_DOT_SIZE),
                        height: app::ui::px_p(HELP_DOT_SIZE),
                        margin: UiRect::new(
                            app::ui::px_p(0.0),
                            app::ui::px_p(4.0),
                            app::ui::px_p(4.0),
                            app::ui::px_p(0.0),
                        ),
                        border: UiRect::all(app::ui::px_p(0.5)),
                        ..default()
                    },
                    background_color: app::ui::BG_COLOR.into(),
                    border_color: app::ui::MUTE_COLOR.into(),
                    ..default()
                },
                HelpDot(0),
            ));
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: app::ui::px_p(HELP_DOT_SIZE),
                        height: app::ui::px_p(HELP_DOT_SIZE),
                        margin: UiRect::new(
                            app::ui::px_p(0.0),
                            app::ui::px_p(4.0),
                            app::ui::px_p(4.0),
                            app::ui::px_p(0.0),
                        ),
                        border: UiRect::all(app::ui::px_p(0.5)),
                        ..default()
                    },
                    background_color: app::ui::BG_COLOR.into(),
                    border_color: app::ui::MUTE_COLOR.into(),
                    ..default()
                },
                HelpDot(0),
            ));
        });
}
