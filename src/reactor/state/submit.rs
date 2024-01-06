use crate::{app, reactor};
use bevy::{input::keyboard, prelude::*};
use bevy_persistent::prelude::*;
use bevy_ui_navigation::{prelude::*, NavRequestSystem};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(reactor::ReactorState::Submit),
            (
                state_setup,
                app::audio::reduce_bgm_volume,
                clear_extra_keyboard_char_events,
            ),
        )
        .add_systems(
            Update,
            (
                handle_keybord_input,
                handle_ui_navigation.after(NavRequestSystem),
            )
                .run_if(in_state(reactor::ReactorState::Submit)),
        )
        .add_systems(
            OnExit(reactor::ReactorState::Submit),
            (app::audio::roll_bgm_volume_back, state_exit),
        );
    }
}

#[derive(Component)]
struct StateRootUi;

#[derive(Component)]
struct PlayerNameInput;

#[derive(Component)]
enum ButtonAction {
    Submit,
    Key(String),
}

const ENDED_BG_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.95);

fn state_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut status: ResMut<reactor::status::ReactorStatus>,
    leaderboard: Res<Persistent<app::leaderboard::Leaderboard>>,
    settings: Res<Persistent<app::settings::Settings>>,
    mut reactor_state: ResMut<NextState<reactor::ReactorState>>,
    mut key_binding: ResMut<app::key_binding::KeyBindingConfig>,
) {
    key_binding.mode = app::key_binding::KeyBindingMode::Keyboard;
    status.mark_timeline("ended");
    let lb_record = status.export();
    let is_new_record = leaderboard.is_new_record(&lb_record);
    if !is_new_record {
        reactor_state.set(reactor::ReactorState::Ended);
        status.highlight_uid = String::from("");
        return;
    }
    status.player_name = String::from(settings.fetch_last_player());
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
                    reactor::state::build_result_panel(
                        parent,
                        &asset_server,
                        &status,
                        &leaderboard,
                    );
                    // parent.spawn(TextBundle::from_section(
                    //     "Game Over",
                    //     TextStyle {
                    //         font: asset_server.load(app::ui::FONT),
                    //         font_size: app::ui::FONT_SIZE * 3.0,
                    //         color: Color::rgba(1.0, 0.0, 0.0, 0.8),
                    //         ..default()
                    //     },
                    // ));
                    // parent
                    //     .spawn((NodeBundle {
                    //         style: Style {
                    //             flex_direction: FlexDirection::Column,
                    //             align_items: AlignItems::Center,
                    //             justify_content: JustifyContent::Center,
                    //             row_gap: app::ui::px_p(4.0),
                    //             ..default()
                    //         },
                    //         ..default()
                    //     },))
                    //     .with_children(|parent| {
                    //         parent
                    //             .spawn((NodeBundle {
                    //                 style: Style {
                    //                     align_items: AlignItems::Center,
                    //                     justify_content: JustifyContent::Center,
                    //                     column_gap: app::ui::px_p(8.0),
                    //                     ..default()
                    //                 },
                    //                 ..default()
                    //             },))
                    //             .with_children(|parent| {
                    //                 build_result_entry(
                    //                     parent,
                    //                     &asset_server,
                    //                     &status,
                    //                     &leaderboard,
                    //                     "score",
                    //                 );
                    //                 build_result_entry(
                    //                     parent,
                    //                     &asset_server,
                    //                     &status,
                    //                     &leaderboard,
                    //                     "time",
                    //                 );
                    //                 build_result_entry(
                    //                     parent,
                    //                     &asset_server,
                    //                     &status,
                    //                     &leaderboard,
                    //                     "max_alpha_count",
                    //                 );
                    //             });
                    //         parent
                    //             .spawn((NodeBundle {
                    //                 style: Style {
                    //                     align_items: AlignItems::Center,
                    //                     justify_content: JustifyContent::Center,
                    //                     column_gap: app::ui::px_p(8.0),
                    //                     ..default()
                    //                 },
                    //                 ..default()
                    //             },))
                    //             .with_children(|parent| {
                    //                 build_result_entry(
                    //                     parent,
                    //                     &asset_server,
                    //                     &status,
                    //                     &leaderboard,
                    //                     "max_control_chain",
                    //                 );
                    //                 build_result_entry(
                    //                     parent,
                    //                     &asset_server,
                    //                     &status,
                    //                     &leaderboard,
                    //                     "max_hyper_chain",
                    //                 );
                    //             });
                    //     });
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
                                        justify_content: JustifyContent::Start,
                                        padding: UiRect::new(
                                            app::ui::px_p(4.0),
                                            app::ui::px_p(3.0),
                                            app::ui::px_p(4.0),
                                            app::ui::px_p(3.0),
                                        ),
                                        border: UiRect::all(app::ui::px_p(1.0)),
                                        ..default()
                                    },
                                    border_color: app::ui::FG_COLOR.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn((
                                        TextBundle::from_section(
                                            format!("{}_", settings.fetch_last_player()),
                                            TextStyle {
                                                font: asset_server.load(app::ui::FONT_DIGIT),
                                                font_size: app::ui::FONT_SIZE,
                                                color: app::ui::FG_COLOR,
                                                ..default()
                                            },
                                        ),
                                        PlayerNameInput,
                                    ));
                                });
                        });
                    build_keyboard(parent, &asset_server);
                    app::ui::build_btn(
                        parent,
                        &asset_server,
                        (
                            ButtonAction::Submit,
                            app::interaction::IaButton,
                            Focusable::new().prioritized(),
                        ),
                        Style {
                            padding: UiRect::all(app::ui::px_p(app::ui::BTN_PADDING)),
                            ..default()
                        },
                        Some("Submit"),
                        Some("download-simple"),
                    );
                });
        });
    dbg!("(submit) status = {}", status);
}

fn state_exit(to_despawn: Query<Entity, With<StateRootUi>>, commands: Commands) {
    app::ui::despawn_ui::<StateRootUi>(to_despawn, commands);
}

fn handle_ui_navigation(
    mut actions: Query<&mut ButtonAction>,
    mut events: EventReader<NavEvent>,
    mut reactor_state: ResMut<NextState<reactor::ReactorState>>,
    mut status: ResMut<reactor::status::ReactorStatus>,
    mut player_name_input: Query<&mut Text, With<PlayerNameInput>>,
    mut leaderboard: ResMut<Persistent<app::leaderboard::Leaderboard>>,
    mut settings: ResMut<Persistent<app::settings::Settings>>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut actions,
        |mut action| match &mut *action {
            ButtonAction::Submit => {
                leaderboard
                    .update(|leaderboard| {
                        let lb_record = status.export();
                        leaderboard.store(lb_record);
                    })
                    .expect("failed to update leaderboard");
                settings
                    .update(|settings| {
                        settings.update_last_player(status.player_name.as_str());
                    })
                    .expect("failed to last player");
                status.highlight_uid = String::from(status.uid());
                reactor_state.set(reactor::ReactorState::Ended);
            }
            ButtonAction::Key(key) => {
                modify_player_name_input_by_key(key.as_str(), &mut status, &mut player_name_input);
            }
        },
    );
}

const KB_ROW_1: [&str; 11] = [
    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
    "G",
    "H",
    "I",
    "backspace",
    "clear",
];
const KB_ROW_2: [&str; 11] = ["J", "K", "L", "N", "M", "O", "P", "Q", "R", "space", "'"];
const KB_ROW_3: [&str; 11] = ["S", "T", "U", "V", "W", "X", "Y", "Z", ".", "-", ","];
const KB_ROWS: [[&str; 11]; 3] = [KB_ROW_1, KB_ROW_2, KB_ROW_3];
const KB_PADDING: f32 = 2.0;
const KB_FS: f32 = app::ui::FONT_SIZE * 0.6;
const KB_KEY_SIZE: f32 = 40.0;
const KB_ICON_SIZE: f32 = 20.0;

fn build_keyboard(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) -> Entity {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: app::ui::px_p(KB_PADDING),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            for kb_row in KB_ROWS {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            column_gap: app::ui::px_p(KB_PADDING),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        for kb_key in kb_row {
                            build_key_btn(parent, &asset_server, kb_key);
                        }
                    });
            }
        })
        .id()
}

fn build_key_btn(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, key: &str) -> Entity {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(KB_KEY_SIZE),
                    height: Val::Px(KB_KEY_SIZE),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: app::ui::BTN_BG.into(),
                ..default()
            },
            (
                ButtonAction::Key(String::from(key)),
                app::interaction::IaButton,
                Focusable::default(),
            ),
        ))
        .with_children(|parent| match key {
            "backspace" | "clear" | "space" => {
                let icon_path = format!("images/icons/key_{}.png", key);
                let icon = asset_server.load(icon_path);
                parent.spawn(ImageBundle {
                    style: Style {
                        width: Val::Px(KB_ICON_SIZE),
                        height: Val::Px(KB_ICON_SIZE),
                        ..default()
                    },
                    image: UiImage::new(icon),
                    ..default()
                });
            }
            _ => {
                parent.spawn(TextBundle::from_section(
                    key,
                    TextStyle {
                        font: asset_server.load(app::ui::FONT_DIGIT),
                        font_size: KB_FS,
                        color: app::ui::FG_COLOR,
                    },
                ));
            }
        })
        .id()
}

fn handle_keybord_input(
    mut events: EventReader<keyboard::KeyboardInput>,
    mut char_events: EventReader<ReceivedCharacter>,
    mut status: ResMut<reactor::status::ReactorStatus>,
    mut player_name_input: Query<&mut Text, With<PlayerNameInput>>,
) {
    use bevy::input::ButtonState;
    for event in events.read() {
        match event.state {
            ButtonState::Pressed => {
                if let Some(key_code) = event.key_code {
                    match key_code {
                        KeyCode::Space => modify_player_name_input_by_key(
                            "space",
                            &mut status,
                            &mut player_name_input,
                        ),
                        KeyCode::Back => modify_player_name_input_by_key(
                            "backspace",
                            &mut status,
                            &mut player_name_input,
                        ),
                        KeyCode::Delete => modify_player_name_input_by_key(
                            "clear",
                            &mut status,
                            &mut player_name_input,
                        ),
                        _ => {
                            for event in char_events.read() {
                                let key = String::from(event.char).to_ascii_uppercase();
                                if KB_ROW_1.contains(&key.as_str())
                                    || KB_ROW_2.contains(&key.as_str())
                                    || KB_ROW_3.contains(&key.as_str())
                                {
                                    modify_player_name_input_by_key(
                                        &key,
                                        &mut status,
                                        &mut player_name_input,
                                    );
                                }
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

fn modify_player_name_input_by_key(
    key: &str,
    status: &mut ResMut<reactor::status::ReactorStatus>,
    player_name_input: &mut Query<&mut Text, With<PlayerNameInput>>,
) {
    match key {
        "backspace" => {
            let mut name_chars = status.player_name.chars();
            name_chars.next_back();
            status.player_name = String::from(name_chars.as_str())
        }
        "clear" => {
            status.player_name = String::from("");
        }
        "space" => {
            if status.player_name.len() >= app::leaderboard::MAX_PLAYER_NAME_LENGTH {
                return;
            }
            status.player_name = format!("{} ", status.player_name);
        }
        _ => {
            if status.player_name.len() >= app::leaderboard::MAX_PLAYER_NAME_LENGTH {
                return;
            }
            status.player_name = format!("{}{}", status.player_name, key);
        }
    }
    for mut text in player_name_input.iter_mut() {
        if status.player_name.len() >= app::leaderboard::MAX_PLAYER_NAME_LENGTH {
            text.sections[0].value = format!("{}", status.player_name);
        } else {
            text.sections[0].value = format!("{}_", status.player_name);
        }
    }
}

fn clear_extra_keyboard_char_events(
    mut keyboard_events: ResMut<Events<keyboard::KeyboardInput>>,
    mut char_events: ResMut<Events<ReceivedCharacter>>,
) {
    if !keyboard_events.is_empty() {
        keyboard_events.clear();
    }
    if !char_events.is_empty() {
        char_events.clear();
    }
}
