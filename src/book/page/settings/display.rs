use super::*;
use crate::app::{anime_effect, element, interaction, ui};
use bevy::window::WindowMode;
use bevy_persistent::prelude::*;
use bevy_ui_navigation::NavRequestSystem;

const PAGE_CODE: &str = "settings_display";
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
        PageState::SettingsDisplay
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
                element::element_systems(),
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
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                build_sep_title(
                                    parent,
                                    &asset_server,
                                    "Fullscreen",
                                    "frame-corners-fill",
                                );
                                element::build_element(
                                    parent,
                                    &asset_server,
                                    ButtonAction::AppUiNav,
                                    element::ElementInitParams::Switcher {
                                        data: element::ElementTargetValuePair {
                                            target: String::from("fullscreen"),
                                            bool_value: Some(settings.is_enabled("fullscreen")),
                                            ..default()
                                        },
                                    },
                                );
                            }
                        });
                    build_settings_nav_bar(parent, &asset_server, PageState::SettingsDisplay);
                });
        });
}

fn handle_ui_navigation(
    action_query: Query<(Entity, &mut ButtonAction), With<ButtonAction>>,
    mut nav_events: EventReader<NavEvent>,
    mut page_state: ResMut<NextState<PageState>>,
) {
    for event in nav_events.read() {
        match event {
            NavEvent::NoChanges { from, request } => match *request {
                NavRequest::Action => {
                    for (entity, action) in action_query.iter() {
                        if *from.first() == entity {
                            match action {
                                ButtonAction::MoveToPage(state) => page_state.set(*state),
                                _ => (),
                            }
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}

fn handle_element_events(
    mut events: EventReader<element::ElementEvent>,
    mut settings: ResMut<Persistent<app::settings::Settings>>,
    mut nav_requests: EventWriter<NavRequest>,
    mut window_query: Query<&mut Window>,
) {
    for event in events.read() {
        match event {
            element::ElementEvent::DataChanged { data } => {
                if let Some(_) = data.bool_value {
                    settings
                        .update(|settings| {
                            settings.toggle(data.target.as_str());
                        })
                        .expect("failed to update switcher");
                    let is_enabled = settings.is_enabled(data.target.as_str());
                    if data.target == "fullscreen" {
                        let mut window = window_query.single_mut();
                        if is_enabled {
                            window.mode = WindowMode::Fullscreen
                        } else {
                            window.mode = WindowMode::Windowed
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
