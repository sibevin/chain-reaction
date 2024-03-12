use super::*;
use crate::app::{anime_effect, element, interaction, ui};
use bevy_persistent::prelude::*;
use bevy_ui_navigation::NavRequestSystem;

const PAGE_CODE: &str = "settings_audio";
const PAGE_NAME: &str = "Variables";
const PAGE_ICON: &str = "gear-light";

pub struct Page;

impl PageBase for Page {
    fn code(&self) -> &str {
        PAGE_CODE
    }
    fn name(&self) -> &str {
        PAGE_NAME
    }
    fn icon(&self) -> &str {
        PAGE_ICON
    }
    fn state(&self) -> PageState {
        PageState::SettingsAudio
    }
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(self.state()),
            (interaction::reset_default_focus, page_enter),
        )
        .add_systems(
            Update,
            ((
                handle_ui_navigation,
                interaction::handle_default_focus,
                element::handle_element_mouse_unlock,
                element::handle_element_mouse_clicking,
                element::handle_element_mouse_dragging,
                element::handle_element_keyboard_pressing,
                element::handle_element_keyboard_changing,
                element::handle_element_gamepad_pressing,
                element::handle_element_gamepad_dpad_changing,
                element::handle_element_gamepad_axis_changing,
                element::handle_element_gamepad_modifier,
                element::handle_element_keyboard_modifier,
                element::refresh_elements,
                handle_element_events,
            )
                .after(NavRequestSystem),)
                .run_if(in_state(self.state())),
        )
        .add_systems(
            OnExit(self.state()),
            (
                anime_effect::clear_anime_effect,
                element::clear_elements,
                ui::despawn_ui::<OnPage>,
            ),
        );
    }
}

#[derive(Component)]
struct InteractionDefaultFocus;

#[derive(Component)]
struct OnPage;

fn page_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<Persistent<app::settings::Settings>>,
) {
    commands
        .spawn((build_page_layout(), OnPage))
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
                    build_game_title(parent, &asset_server);
                    build_page_title(parent, &asset_server, PAGE_NAME, PAGE_ICON);
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_grow: 1.0,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            build_sep_title(parent, &asset_server, "BGM", "music-notes-fill");
                            element::build_element(
                                parent,
                                &asset_server,
                                ButtonAction::AppUiNav,
                                element::ElementInitParams::Switcher {
                                    data: element::ElementTargetValuePair {
                                        target: String::from("bgm"),
                                        bool_value: Some(settings.is_enabled("bgm")),
                                        ..default()
                                    },
                                },
                            );
                            element::build_element(
                                parent,
                                &asset_server,
                                ButtonAction::AppUiNav,
                                element::ElementInitParams::Slider {
                                    data: element::ElementTargetValuePair {
                                        target: String::from("bgm"),
                                        u8_value: Some(settings.get_value("bgm")),
                                        ..default()
                                    },
                                },
                            );
                            build_sep_title(parent, &asset_server, "SE", "waveform-fill");
                            element::build_element(
                                parent,
                                &asset_server,
                                ButtonAction::AppUiNav,
                                element::ElementInitParams::Switcher {
                                    data: element::ElementTargetValuePair {
                                        target: String::from("se"),
                                        bool_value: Some(settings.is_enabled("se")),
                                        ..default()
                                    },
                                },
                            );
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        align_items: AlignItems::Center,
                                        column_gap: ui::px_p(4.0),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    element::build_element(
                                        parent,
                                        &asset_server,
                                        ButtonAction::AppUiNav,
                                        element::ElementInitParams::Slider {
                                            data: element::ElementTargetValuePair {
                                                target: String::from("se"),
                                                u8_value: Some(settings.get_value("se")),
                                                ..default()
                                            },
                                        },
                                    );
                                    ui::build_btn(
                                        parent,
                                        &asset_server,
                                        (
                                            ButtonAction::PlaySe,
                                            app::interaction::IaButton,
                                            Focusable::default(),
                                        ),
                                        Style {
                                            padding: UiRect::all(ui::px_p(ui::BTN_PADDING)),
                                            ..default()
                                        },
                                        None,
                                        Some("play-light"),
                                    );
                                });
                        });
                    build_settings_nav_bar(parent, &asset_server, PageState::SettingsAudio);
                });
        });
}

fn handle_element_events(
    mut events: EventReader<element::ElementEvent>,
    mut settings: ResMut<Persistent<app::settings::Settings>>,
    audio_bgm_query: app::audio::QueryAudioBgm,
    mut ele_query: Query<(Entity, &mut element::ElementData), With<element::ElementData>>,
    mut nav_requests: EventWriter<NavRequest>,
) {
    for event in events.read() {
        match event {
            element::ElementEvent::DataChanged { data } => {
                dbg!(data);
                if let Some(u8_value) = data.u8_value {
                    settings
                        .update(|settings| {
                            settings.set_value(data.target.as_str(), u8_value as i8);
                        })
                        .expect("failed to update slider");
                    if data.target == "bgm" {
                        app::audio::set_bgm_volume(settings.get_value("bgm"), &audio_bgm_query);
                    }
                    element::update_element_value(&mut ele_query, data.clone());
                }
                if let Some(_) = data.bool_value {
                    settings
                        .update(|settings| {
                            settings.toggle(data.target.as_str());
                        })
                        .expect("failed to update switcher");
                    let is_enabled = settings.is_enabled(data.target.as_str());
                    if data.target == "bgm" {
                        if let Ok(sink) = audio_bgm_query.get_single() {
                            if is_enabled {
                                sink.play();
                            } else {
                                sink.pause();
                            }
                        }
                    }
                }
            }
            element::ElementEvent::Lock { entity: _ } => {
                nav_requests.send(NavRequest::Lock);
            }
            element::ElementEvent::Unlock => {
                nav_requests.send(NavRequest::Unlock);
            }
            _ => (),
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_ui_navigation(
    mut commands: Commands,
    action_query: Query<(Entity, &mut ButtonAction), With<ButtonAction>>,
    mut nav_events: EventReader<NavEvent>,
    mut page_state: ResMut<NextState<PageState>>,
    settings: Res<Persistent<app::settings::Settings>>,
    mut ele_query: Query<(Entity, &mut element::ElementData), With<element::ElementData>>,
    asset_server: Res<AssetServer>,
) {
    for event in nav_events.read() {
        match event {
            NavEvent::NoChanges { from, request } => match *request {
                NavRequest::Action => {
                    for (entity, action) in action_query.iter() {
                        if *from.first() == entity {
                            match action {
                                ButtonAction::PlaySe => {
                                    app::audio::play_se(
                                        "game_over",
                                        &mut commands,
                                        &asset_server,
                                        settings.as_ref(),
                                    );
                                }
                                ButtonAction::MoveToPage(state) => page_state.set(*state),
                                _ => (),
                            }
                        }
                    }
                }
                NavRequest::Unlock => {
                    element::apply_element_lock(None, &mut ele_query);
                }
                _ => (),
            },
            _ => (),
        }
    }
}
