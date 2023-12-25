use crate::{app, reactor};
use bevy::prelude::*;
use bevy_persistent::prelude::*;
use bevy_ui_navigation::{prelude::*, NavRequestSystem};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(reactor::ReactorState::Ended),
            (state_setup, app::audio::reduce_bgm_volume),
        )
        .add_systems(
            Update,
            handle_ui_navigation
                .after(NavRequestSystem)
                .run_if(in_state(reactor::ReactorState::Ended)),
        )
        .add_systems(
            OnExit(reactor::ReactorState::Ended),
            (app::audio::roll_bgm_volume_back, state_exit),
        );
    }
}

#[derive(Component)]
struct StateRootUi;

#[derive(Component)]
enum ButtonAction {
    ReStart,
    Leaderboard,
    BackToMenu,
}

const ENDED_BG_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.95);

fn state_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    status: Res<reactor::status::ReactorStatus>,
    leaderboard: Res<Persistent<app::leaderboard::Leaderboard>>,
    mut key_binding: ResMut<app::key_binding::KeyBindingConfig>,
) {
    key_binding.mode = app::key_binding::KeyBindingMode::Navgation;
    let lb_record = status.export();
    let is_new_record = leaderboard.is_new_record(&lb_record);
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: ENDED_BG_COLOR.into(),
                ..default()
            },
            StateRootUi,
        ))
        .with_children(|parent| {
            parent
                .spawn((NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        row_gap: app::ui::px_p(8.0),
                        ..default()
                    },
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Game Over",
                        TextStyle {
                            font: asset_server.load(app::ui::FONT),
                            font_size: app::ui::FONT_SIZE * 3.0,
                            color: Color::rgba(1.0, 0.0, 0.0, 0.8),
                            ..default()
                        },
                    ));
                    parent
                        .spawn((NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                row_gap: app::ui::px_p(4.0),
                                ..default()
                            },
                            ..default()
                        },))
                        .with_children(|parent| {
                            parent
                                .spawn((NodeBundle {
                                    style: Style {
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        column_gap: app::ui::px_p(8.0),
                                        ..default()
                                    },
                                    ..default()
                                },))
                                .with_children(|parent| {
                                    build_result_entry(
                                        parent,
                                        &asset_server,
                                        &status,
                                        &leaderboard,
                                        "score",
                                    );
                                    build_result_entry(
                                        parent,
                                        &asset_server,
                                        &status,
                                        &leaderboard,
                                        "time",
                                    );
                                    build_result_entry(
                                        parent,
                                        &asset_server,
                                        &status,
                                        &leaderboard,
                                        "max_alpha_count",
                                    );
                                });
                            parent
                                .spawn((NodeBundle {
                                    style: Style {
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        column_gap: app::ui::px_p(8.0),
                                        ..default()
                                    },
                                    ..default()
                                },))
                                .with_children(|parent| {
                                    build_result_entry(
                                        parent,
                                        &asset_server,
                                        &status,
                                        &leaderboard,
                                        "max_control_chain",
                                    );
                                    build_result_entry(
                                        parent,
                                        &asset_server,
                                        &status,
                                        &leaderboard,
                                        "max_hyper_chain",
                                    );
                                });
                        });
                    if is_new_record {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    column_gap: app::ui::px_p(4.0),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: app::ui::px_p(90.0),
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            padding: UiRect::all(app::ui::px_p(3.0)),
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn(TextBundle::from_section(
                                            String::from(status.player_name.as_str()),
                                            TextStyle {
                                                font: asset_server.load(app::ui::FONT_DIGIT),
                                                font_size: app::ui::FONT_SIZE,
                                                color: app::ui::FG_COLOR,
                                                ..default()
                                            },
                                        ));
                                    });
                            });
                    }
                    parent
                        .spawn((NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                column_gap: app::ui::px_p(4.0),
                                ..default()
                            },
                            ..default()
                        },))
                        .with_children(|parent| {
                            app::ui::build_btn(
                                parent,
                                &asset_server,
                                (
                                    ButtonAction::ReStart,
                                    app::interaction::IaButton,
                                    Focusable::new().prioritized(),
                                ),
                                Style {
                                    padding: UiRect::all(app::ui::px_p(app::ui::BTN_PADDING)),
                                    ..default()
                                },
                                Some("Re-Start"),
                                Some("arrow-counter-clockwise"),
                            );
                            app::ui::build_btn(
                                parent,
                                &asset_server,
                                (
                                    ButtonAction::Leaderboard,
                                    app::interaction::IaButton,
                                    Focusable::default(),
                                ),
                                Style {
                                    padding: UiRect::all(app::ui::px_p(app::ui::BTN_PADDING)),
                                    ..default()
                                },
                                Some("Report"),
                                Some("list-numbers"),
                            );
                        });
                });
            app::ui::build_icon_btn(
                parent,
                &asset_server,
                (
                    ButtonAction::BackToMenu,
                    app::interaction::IaButton,
                    Focusable::default(),
                ),
                Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(18.0),
                    bottom: Val::Px(18.0),
                    ..default()
                },
                "arrow-left-light",
            );
        });
}

fn state_exit(to_despawn: Query<Entity, With<StateRootUi>>, commands: Commands) {
    app::ui::despawn_ui::<StateRootUi>(to_despawn, commands);
}

fn handle_ui_navigation(
    mut actions: Query<&mut ButtonAction>,
    mut events: EventReader<NavEvent>,
    mut game_state: ResMut<NextState<app::GameState>>,
    mut reactor_state: ResMut<NextState<reactor::ReactorState>>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut actions,
        |mut action| match &mut *action {
            ButtonAction::BackToMenu => game_state.set(app::GameState::Menu),
            ButtonAction::ReStart => reactor_state.set(reactor::ReactorState::Ready),
            ButtonAction::Leaderboard => game_state.set(app::GameState::Leaderboard),
        },
    );
}

const RESULT_ICON_SIZE: f32 = 12.0;
const RESULT_PADDING: f32 = 2.0;
const RESULT_FS: f32 = app::ui::FONT_SIZE * 0.8;

fn build_result_entry(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    status: &Res<reactor::status::ReactorStatus>,
    leaderboard: &Res<Persistent<app::leaderboard::Leaderboard>>,
    field: &str,
) -> Entity {
    parent
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            if field == "max_control_chain" || field == "max_hyper_chain" {
                let icon = asset_server.load("images/icons/line-segments.png");
                parent.spawn((ImageBundle {
                    style: Style {
                        width: app::ui::px_p(RESULT_ICON_SIZE),
                        height: app::ui::px_p(RESULT_ICON_SIZE),
                        margin: UiRect::right(app::ui::px_p(RESULT_PADDING)),
                        ..default()
                    },
                    image: UiImage::new(icon),
                    ..default()
                },));
            }
            let icon = match field {
                "time" => asset_server.load("images/icons/timer-fill.png"),
                "score" => asset_server.load("images/icons/trophy-fill.png"),
                "max_alpha_count" => asset_server.load("images/icons/circles-three-fill.png"),
                "max_control_chain" => asset_server.load("images/icons/square.png"),
                "max_hyper_chain" => asset_server.load("images/icons/hexagon.png"),
                _ => panic!("Invalid field"),
            };
            parent.spawn(ImageBundle {
                style: Style {
                    width: app::ui::px_p(RESULT_ICON_SIZE),
                    height: app::ui::px_p(RESULT_ICON_SIZE),
                    margin: UiRect::right(app::ui::px_p(RESULT_PADDING)),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Start,
                        justify_content: JustifyContent::Center,
                        row_gap: app::ui::px_p(RESULT_PADDING),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    let rank: u8 = leaderboard.rank(field, status.fetch(field));
                    let rank_color = if rank <= 10 {
                        app::ui::FG_COLOR
                    } else {
                        app::ui::SECONDARY_COLOR
                    };
                    let rank_text = match rank {
                        1 => String::from("1st"),
                        2 => String::from("2nd"),
                        3 => String::from("3rd"),
                        4 | 5 | 6 | 7 | 8 | 9 | 10 => format!("{}th", rank),
                        _ => String::from("---"),
                    };
                    let is_new = leaderboard.is_new_in_list(field, status.fetch(field));
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Start,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    background_color: rank_color.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(
                                        TextBundle::from_section(
                                            rank_text,
                                            TextStyle {
                                                font: asset_server.load(app::ui::FONT_DIGIT),
                                                font_size: RESULT_FS * 0.6,
                                                color: app::ui::BG_COLOR,
                                                ..default()
                                            },
                                        )
                                        .with_style(
                                            Style {
                                                margin: UiRect::all(app::ui::px_p(
                                                    RESULT_PADDING * 0.5,
                                                )),
                                                ..default()
                                            },
                                        ),
                                    );
                                });
                            if is_new {
                                parent.spawn(
                                    TextBundle::from_section(
                                        "NEW!!",
                                        TextStyle {
                                            font: asset_server.load(app::ui::FONT_DIGIT),
                                            font_size: RESULT_FS * 0.6,
                                            color: app::ui::FG_COLOR,
                                            ..default()
                                        },
                                    )
                                    .with_style(Style {
                                        margin: UiRect::all(app::ui::px_p(RESULT_PADDING * 0.5)),
                                        ..default()
                                    }),
                                );
                            }
                        });
                    let format_field = match field {
                        "time" | "score" => field,
                        "max_alpha_count" => "alpha_count",
                        "max_control_chain" | "max_hyper_chain" => "chain",
                        _ => panic!("Invalid field"),
                    };
                    let text_color = match field {
                        "time" | "score" | "max_alpha_count" => app::ui::SECONDARY_COLOR,
                        "max_control_chain" => reactor::particle::control::COLOR,
                        "max_hyper_chain" => reactor::particle::hyper::COLOR,
                        _ => panic!("Invalid field"),
                    };
                    parent.spawn(TextBundle::from_section(
                        reactor::field::format_field_text(format_field, status.fetch(field)),
                        TextStyle {
                            font: asset_server.load(app::ui::FONT_DIGIT),
                            font_size: RESULT_FS,
                            color: text_color,
                            ..default()
                        },
                    ));
                });
        })
        .id()
}
