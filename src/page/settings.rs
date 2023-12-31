use bevy::{input, prelude::*, window::WindowMode};
use bevy_persistent::prelude::*;
use bevy_ui_navigation::{prelude::*, NavRequestSystem};

use crate::{app, page, reactor};

pub struct PagePlugin;

impl Plugin for PagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(app::GameState::Settings), page_setup)
            .add_systems(
                Update,
                (
                    move_test_panel_action,
                    control_test_ball_by_keyboard,
                    (
                        handle_slider_mouse_interaction,
                        handle_ui_navigation,
                        handle_slider_navigation,
                    )
                        .after(NavRequestSystem),
                )
                    .run_if(in_state(app::GameState::Settings)),
            )
            .add_systems(
                OnExit(app::GameState::Settings),
                app::ui::despawn_ui::<OnPage>,
            );
    }
}

#[derive(Component)]
struct OnPage;

#[derive(Component)]
struct SwitchButtonIcon;

#[derive(Component)]
struct RangeValueBar;

#[derive(Component)]
struct RangeBgBar;

#[derive(Component)]
struct RangeBarText;

#[derive(Component)]
struct MoveTestPanel;

#[derive(Component)]
struct MoveTestBall;

#[derive(Component, Debug)]
enum ButtonAction {
    BackToMainMenu,
    Toggle(String),
    SetValue(String),
    PlaySe,
}

type RangeBgBarOnly = (With<RangeBgBar>, Without<RangeValueBar>);
type RangeValueBarOnly = (With<RangeValueBar>, Without<RangeBgBar>);

fn page_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<Persistent<app::settings::Settings>>,
) {
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
                    page::build_page_title(parent, &asset_server, "Variables", "gear-light");
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_grow: 1.0,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                margin: UiRect::top(app::ui::px_p(24.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Start,
                                        justify_content: JustifyContent::Center,
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
                                                margin: UiRect::right(app::ui::px_p(10.0)),
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            page::build_sep_title(
                                                parent,
                                                &asset_server,
                                                "BGM",
                                                "music-notes-fill",
                                            );
                                            build_switch_btn(
                                                parent,
                                                &asset_server,
                                                ButtonAction::Toggle(String::from("bgm")),
                                                settings.is_enabled("bgm"),
                                            );
                                            build_slider_bar(
                                                parent,
                                                &asset_server,
                                                ButtonAction::SetValue(String::from("bgm")),
                                                settings.get_value("bgm"),
                                            );
                                            page::build_sep_title(
                                                parent,
                                                &asset_server,
                                                "SE",
                                                "waveform-fill",
                                            );
                                            build_switch_btn(
                                                parent,
                                                &asset_server,
                                                ButtonAction::Toggle(String::from("se")),
                                                settings.is_enabled("se"),
                                            );
                                            parent
                                                .spawn(NodeBundle {
                                                    style: Style {
                                                        align_items: AlignItems::Center,
                                                        column_gap: app::ui::px_p(4.0),
                                                        ..default()
                                                    },
                                                    ..default()
                                                })
                                                .with_children(|parent| {
                                                    build_slider_bar(
                                                        parent,
                                                        &asset_server,
                                                        ButtonAction::SetValue(String::from("se")),
                                                        settings.get_value("se"),
                                                    );
                                                    app::ui::build_icon_btn(
                                                        parent,
                                                        &asset_server,
                                                        (
                                                            ButtonAction::PlaySe,
                                                            app::interaction::IaButton,
                                                            Focusable::default(),
                                                        ),
                                                        Style::default(),
                                                        "play-light",
                                                    );
                                                });
                                            #[cfg(not(target_arch = "wasm32"))]
                                            {
                                                page::build_sep_title(
                                                    parent,
                                                    &asset_server,
                                                    "Fullscreen",
                                                    "frame-corners-fill",
                                                );
                                                build_switch_btn(
                                                    parent,
                                                    &asset_server,
                                                    ButtonAction::Toggle(String::from(
                                                        "fullscreen",
                                                    )),
                                                    settings.is_enabled("fullscreen"),
                                                );
                                            }
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
                                            page::build_sep_title(
                                                parent,
                                                &asset_server,
                                                "Sensitivity",
                                                "gauge-fill",
                                            );
                                            build_slider_bar(
                                                parent,
                                                &asset_server,
                                                ButtonAction::SetValue(String::from("sensitivity")),
                                                settings.get_value("sensitivity"),
                                            );
                                            build_move_testing_panel(parent)
                                        });
                                });
                        });
                    app::ui::build_icon_btn(
                        parent,
                        &asset_server,
                        (
                            ButtonAction::BackToMainMenu,
                            app::interaction::IaButton,
                            Focusable::new().prioritized(),
                        ),
                        Style {
                            align_self: AlignSelf::Start,
                            ..default()
                        },
                        "arrow-left-light",
                    );
                });
        });
}

fn handle_slider_mouse_interaction(
    range_bar_query: Query<
        (&Interaction, &ButtonAction, &Children),
        With<app::interaction::IaSlider>,
    >,
    mut range_value_bar_query: Query<(Entity, &mut Style), RangeValueBarOnly>,
    mut range_bg_bar_query: Query<(Entity, &mut Style), RangeBgBarOnly>,
    mut range_bar_text_query: Query<(Entity, &mut Text), With<RangeBarText>>,
    mut settings: ResMut<Persistent<app::settings::Settings>>,
    mut mouse_motion_events: EventReader<input::mouse::MouseMotion>,
    audio_bgm_query: Query<&AudioSink, With<app::audio::AudioBgm>>,
) {
    for (interaction, action, children) in &range_bar_query {
        if let ButtonAction::SetValue(target) = action {
            if *interaction == Interaction::Pressed {
                let events = mouse_motion_events.read().collect::<Vec<_>>();
                if let Some(event) = events.iter().rev().take(3).next() {
                    update_slider_display(
                        children,
                        target,
                        (event.delta.x * 0.5) as i8,
                        &mut settings,
                        &mut range_value_bar_query,
                        &mut range_bg_bar_query,
                        &mut range_bar_text_query,
                    );
                    if target == "bgm" {
                        update_bgm_volume(&mut settings, &audio_bgm_query);
                    }
                }
            }
        }
    }
}

fn move_test_panel_action(
    mut panel_query: Query<(&Interaction, &Children), With<MoveTestPanel>>,
    mut ball_query: Query<&mut Style, With<MoveTestBall>>,
    mut mouse_motion_events: EventReader<input::mouse::MouseMotion>,
    settings: Res<Persistent<app::settings::Settings>>,
) {
    for (interaction, children) in &mut panel_query {
        if *interaction == Interaction::Pressed {
            let events = mouse_motion_events.read().collect::<Vec<_>>();
            if let Some(event) = events.iter().rev().take(3).next() {
                let mut ball_style = ball_query.get_mut(children[0]).unwrap();
                let ori_x: f32 = match ball_style.left {
                    Val::Px(value) => value,
                    _ => 0.,
                };
                let ori_y: f32 = match ball_style.top {
                    Val::Px(value) => value,
                    _ => 0.,
                };

                let new_pos = calculate_test_ball_pos(
                    (ori_x, ori_y),
                    event.delta,
                    settings.get_value("sensitivity"),
                );
                ball_style.left = Val::Px(new_pos.0);
                ball_style.top = Val::Px(new_pos.1);
            }
        }
    }
}

const SWITCH_ICON_RATIO: f32 = 1.6;

fn build_switch_btn(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    bundle: impl Bundle,
    init_value: bool,
) -> Entity {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(app::ui::px_p(1.0)),
                    padding: UiRect::all(app::ui::px_p(3.0)),
                    ..default()
                },
                background_color: app::ui::BG_COLOR.into(),
                ..default()
            },
            bundle,
            app::interaction::IaSwitch,
            Focusable::default(),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "ON",
                TextStyle {
                    font: asset_server.load(app::ui::FONT),
                    font_size: app::ui::FONT_SIZE,
                    color: app::ui::FG_COLOR,
                },
            ));
            let icon = if init_value {
                asset_server.load("images/icons/toggle-left-fill.png")
            } else {
                asset_server.load("images/icons/toggle-right-fill.png")
            };
            parent.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Px(app::ui::ICON_SIZE * SWITCH_ICON_RATIO),
                        height: Val::Px(app::ui::ICON_SIZE * SWITCH_ICON_RATIO),
                        margin: UiRect::horizontal(app::ui::px_p(6.0)),
                        ..default()
                    },
                    image: UiImage::new(icon),
                    ..default()
                },
                SwitchButtonIcon,
            ));
            parent.spawn(TextBundle::from_section(
                "OFF",
                TextStyle {
                    font: asset_server.load(app::ui::FONT),
                    font_size: app::ui::FONT_SIZE,
                    color: app::ui::MUTE_COLOR,
                },
            ));
        })
        .id()
}

const RANGE_BAR_H: f32 = 6.0;
const RANGE_BAR_W: f32 = 60.0;

fn build_slider_bar(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    bundle: impl Bundle,
    init_value: u8,
) -> Entity {
    let range_bar_w = calculate_range_bar_width(init_value);
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(app::ui::px_p(4.0)),
                    border: UiRect::all(app::ui::px_p(1.0)),
                    ..default()
                },
                background_color: app::ui::BG_COLOR.into(),
                ..default()
            },
            bundle,
            app::interaction::IaSlider,
            Focusable::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: app::ui::px_p(range_bar_w.0),
                        height: app::ui::px_p(RANGE_BAR_H),
                        ..default()
                    },
                    background_color: app::ui::FG_COLOR.into(),
                    ..default()
                },
                RangeValueBar,
            ));
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: app::ui::px_p(range_bar_w.1),
                        height: app::ui::px_p(RANGE_BAR_H),
                        ..default()
                    },
                    background_color: app::ui::MUTE_COLOR.into(),
                    ..default()
                },
                RangeBgBar,
            ));
            parent.spawn((
                TextBundle::from_section(
                    format!("{}", init_value),
                    TextStyle {
                        font: asset_server.load(app::ui::FONT),
                        font_size: app::ui::FONT_SIZE,
                        color: app::ui::FG_COLOR,
                    },
                )
                .with_style(Style {
                    margin: UiRect::new(
                        app::ui::px_p(6.0),
                        app::ui::px_p(0.0),
                        app::ui::px_p(0.0),
                        app::ui::px_p(1.0),
                    ),
                    ..default()
                }),
                RangeBarText,
            ));
        })
        .id()
}

fn calculate_range_bar_width(value: u8) -> (f32, f32) {
    let value_bar_w = value as f32 / 100.0 * RANGE_BAR_W;
    (value_bar_w, RANGE_BAR_W - value_bar_w)
}

const MTP_PANEL_SIZE: f32 = 100.0;
const MTP_BALL_SIZE: f32 = 5.0;
const MTP_BALL_POS: f32 = (MTP_PANEL_SIZE - MTP_BALL_SIZE) / 2.0;

fn build_move_testing_panel(parent: &mut ChildBuilder) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    width: app::ui::px_p(MTP_PANEL_SIZE),
                    height: app::ui::px_p(MTP_PANEL_SIZE),
                    margin: UiRect::top(app::ui::px_p(6.0)),
                    border: UiRect::all(app::ui::px_p(0.5)),
                    ..default()
                },
                border_color: app::ui::SECONDARY_COLOR.into(),
                ..default()
            },
            Interaction::default(),
            MoveTestPanel,
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: app::ui::px_p(MTP_BALL_SIZE),
                        height: app::ui::px_p(MTP_BALL_SIZE),
                        top: app::ui::px_p(MTP_BALL_POS),
                        left: app::ui::px_p(MTP_BALL_POS),
                        ..default()
                    },
                    background_color: reactor::U_COLOR.into(),
                    ..default()
                },
                MoveTestBall,
            ));
        });
}

fn calculate_test_ball_pos(current: (f32, f32), delta: Vec2, sensitivity: u8) -> (f32, f32) {
    let delta_ratio = 0.5 + sensitivity as f32 / 100.0 * 5.0;
    let max_value = (MTP_PANEL_SIZE - MTP_BALL_SIZE) * app::ui::SPACE_SIZE - 4.0;
    let new_x = (current.0 + delta.x * delta_ratio).clamp(0., max_value);
    let new_y = (current.1 + delta.y * delta_ratio).clamp(0., max_value);
    (new_x, new_y)
}

const KEYBOARD_DELTA_BIAS: f32 = 1.5;

fn control_test_ball_by_keyboard(
    keyboard_input: Res<Input<KeyCode>>,
    panel_query: Query<(&Interaction, &Children), With<MoveTestPanel>>,
    mut ball_query: Query<&mut Style, With<MoveTestBall>>,
    settings: Res<Persistent<app::settings::Settings>>,
) {
    let mut delta: Vec2 = Vec2::default();
    if keyboard_input.pressed(KeyCode::W)
        || keyboard_input.pressed(KeyCode::Up)
        || keyboard_input.pressed(KeyCode::K)
    {
        delta.y = -KEYBOARD_DELTA_BIAS;
    }
    if keyboard_input.pressed(KeyCode::S)
        || keyboard_input.pressed(KeyCode::Down)
        || keyboard_input.pressed(KeyCode::J)
    {
        delta.y = KEYBOARD_DELTA_BIAS;
    }
    if keyboard_input.pressed(KeyCode::A)
        || keyboard_input.pressed(KeyCode::Left)
        || keyboard_input.pressed(KeyCode::H)
    {
        delta.x = -KEYBOARD_DELTA_BIAS;
    }
    if keyboard_input.pressed(KeyCode::D)
        || keyboard_input.pressed(KeyCode::Right)
        || keyboard_input.pressed(KeyCode::L)
    {
        delta.x = KEYBOARD_DELTA_BIAS;
    }
    let (_, children) = panel_query.single();
    let mut ball_style = ball_query.get_mut(children[0]).unwrap();
    let ori_x: f32 = match ball_style.left {
        Val::Px(value) => value,
        _ => 0.,
    };
    let ori_y: f32 = match ball_style.top {
        Val::Px(value) => value,
        _ => 0.,
    };

    let new_pos = calculate_test_ball_pos((ori_x, ori_y), delta, settings.get_value("sensitivity"));
    ball_style.left = Val::Px(new_pos.0);
    ball_style.top = Val::Px(new_pos.1);
}

const SLIDER_DELTA: i8 = 5;

fn handle_slider_navigation(
    mut events: EventReader<NavEvent>,
    mut action_query: Query<(&ButtonAction, &Children), With<app::interaction::IaSlider>>,
    mut settings: ResMut<Persistent<app::settings::Settings>>,
    mut range_value_bar_query: Query<(Entity, &mut Style), RangeValueBarOnly>,
    mut range_bg_bar_query: Query<(Entity, &mut Style), RangeBgBarOnly>,
    mut range_bar_text_query: Query<(Entity, &mut Text), With<RangeBarText>>,
    audio_bgm_query: Query<&AudioSink, With<app::audio::AudioBgm>>,
) {
    for event in events.read() {
        if let NavEvent::NoChanges { from, request } = event {
            match request {
                NavRequest::Action => {
                    for (action, children) in action_query.iter_many_mut(vec![*from.first()]) {
                        if let ButtonAction::SetValue(target) = action {
                            update_slider_display(
                                children,
                                target,
                                SLIDER_DELTA,
                                &mut settings,
                                &mut range_value_bar_query,
                                &mut range_bg_bar_query,
                                &mut range_bar_text_query,
                            );
                            if target == "bgm" {
                                update_bgm_volume(&mut settings, &audio_bgm_query);
                            }
                        }
                    }
                }
                NavRequest::Cancel => {
                    for (action, children) in action_query.iter_many_mut(vec![*from.first()]) {
                        if let ButtonAction::SetValue(target) = action {
                            update_slider_display(
                                children,
                                target,
                                -SLIDER_DELTA,
                                &mut settings,
                                &mut range_value_bar_query,
                                &mut range_bg_bar_query,
                                &mut range_bar_text_query,
                            );
                            if target == "bgm" {
                                update_bgm_volume(&mut settings, &audio_bgm_query);
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }
}

fn update_slider_display(
    children: &Children,
    target: &str,
    delta: i8,
    settings: &mut ResMut<Persistent<app::settings::Settings>>,
    range_value_bar_query: &mut Query<(Entity, &mut Style), RangeValueBarOnly>,
    range_bg_bar_query: &mut Query<(Entity, &mut Style), RangeBgBarOnly>,
    range_bar_text_query: &mut Query<(Entity, &mut Text), With<RangeBarText>>,
) {
    let updated_value = settings.get_value(target) as i8 + delta;
    settings
        .update(|settings| {
            settings.set_value(target, updated_value);
        })
        .expect("failed to update slider");
    let value = settings.get_value(target);
    let range_bar_w = calculate_range_bar_width(value);
    for child in children {
        for (bar_entity, mut bar_style) in range_value_bar_query.iter_mut() {
            if *child == bar_entity {
                bar_style.width = app::ui::px_p(range_bar_w.0);
                break;
            }
        }
        for (bar_entity, mut bar_style) in range_bg_bar_query.iter_mut() {
            if *child == bar_entity {
                bar_style.width = app::ui::px_p(range_bar_w.1);
                break;
            }
        }
        for (bar_entity, mut bar_text) in range_bar_text_query.iter_mut() {
            if *child == bar_entity {
                bar_text.sections[0].value = format!("{}", value);
                break;
            }
        }
    }
}

fn update_bgm_volume(
    settings: &mut ResMut<Persistent<app::settings::Settings>>,
    audio_bgm_query: &Query<&AudioSink, With<app::audio::AudioBgm>>,
) {
    let value = settings.get_value("bgm");
    if let Ok(sink) = audio_bgm_query.get_single() {
        sink.set_volume(app::audio::to_volume(value));
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_ui_navigation(
    mut action_query: Query<(&mut ButtonAction, &Children), Without<app::interaction::IaSlider>>,
    mut events: EventReader<NavEvent>,
    mut commands: Commands,
    mut game_state: ResMut<NextState<app::GameState>>,
    mut settings: ResMut<Persistent<app::settings::Settings>>,
    mut window_query: Query<&mut Window>,
    mut switch_icon_query: Query<(Entity, &mut UiImage), With<SwitchButtonIcon>>,
    asset_server: Res<AssetServer>,
    audio_bgm_query: Query<&AudioSink, With<app::audio::AudioBgm>>,
    audio_se_asset: Res<app::audio::AudioSeAsset>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut action_query,
        |(mut action, children)| match &mut *action {
            ButtonAction::Toggle(target) => {
                settings
                    .update(|settings| {
                        settings.toggle(target.as_ref());
                    })
                    .expect("failed to update boolean switch");
                let is_enabled = settings.is_enabled(target);
                let mut is_found = false;
                for (icon_entity, mut icon_image) in switch_icon_query.iter_mut() {
                    for child in children {
                        if *child == icon_entity {
                            is_found = true;
                            icon_image.texture = if is_enabled {
                                asset_server.load("images/icons/toggle-left-fill.png")
                            } else {
                                asset_server.load("images/icons/toggle-right-fill.png")
                            };
                        }
                        if is_found {
                            break;
                        }
                    }
                    if is_found {
                        break;
                    }
                }
                if target == "fullscreen" {
                    let mut window = window_query.single_mut();
                    if is_enabled {
                        window.mode = WindowMode::Fullscreen
                    } else {
                        window.mode = WindowMode::Windowed
                    }
                } else if target == "bgm" {
                    if let Ok(sink) = audio_bgm_query.get_single() {
                        if is_enabled {
                            sink.play();
                        } else {
                            sink.pause();
                        }
                    }
                }
            }
            ButtonAction::PlaySe => {
                app::audio::play_se(
                    app::audio::AudioSe::Boom,
                    &mut commands,
                    &audio_se_asset,
                    settings.as_ref(),
                );
            }
            ButtonAction::BackToMainMenu => game_state.set(app::GameState::Menu),
            _ => (),
        },
    );
}
