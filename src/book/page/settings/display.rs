use super::*;
use crate::{app::anime_effect, app::interaction, app::ui};
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
            (
                interaction::reset_default_focus,
                ui::clear_ui_canvas,
                page_enter,
            ),
        )
        .add_systems(
            Update,
            ((handle_ui_navigation, interaction::handle_default_focus).after(NavRequestSystem),)
                .run_if(in_state(self.state())),
        )
        .add_systems(
            OnExit(self.state()),
            (anime_effect::clear_anime_effect, ui::despawn_ui::<OnPage>),
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
                                ui::build_switch_btn(
                                    parent,
                                    &asset_server,
                                    ButtonAction::Toggle(String::from("fullscreen")),
                                    settings.is_enabled("fullscreen"),
                                );
                            }
                        });
                    build_settings_nav_bar(parent, &asset_server, PageState::SettingsDisplay);
                });
        });
}

#[allow(clippy::too_many_arguments)]
fn handle_ui_navigation(
    action_query: Query<(Entity, &mut ButtonAction), With<ButtonAction>>,
    mut switch_btn_query: Query<(&Parent, &mut UiImage, &mut ui::SwitchButton)>,
    mut nav_events: EventReader<NavEvent>,
    mut page_state: ResMut<NextState<PageState>>,
    mut settings: ResMut<Persistent<app::settings::Settings>>,
    mut window_query: Query<&mut Window>,
    asset_server: Res<AssetServer>,
) {
    for event in nav_events.read() {
        match event {
            NavEvent::NoChanges { from, request } => match *request {
                NavRequest::Action => {
                    for (entity, action) in action_query.iter() {
                        if *from.first() == entity {
                            match action {
                                ButtonAction::Toggle(target) => {
                                    settings
                                        .update(|settings| {
                                            settings.toggle(target.as_ref());
                                        })
                                        .expect("failed to update boolean switch");
                                    let is_enabled = settings.is_enabled(target);
                                    ui::update_switch_btn_value(
                                        entity,
                                        &mut switch_btn_query,
                                        &asset_server,
                                        is_enabled,
                                    );
                                    if target == "fullscreen" {
                                        let mut window = window_query.single_mut();
                                        if is_enabled {
                                            window.mode = WindowMode::Fullscreen
                                        } else {
                                            window.mode = WindowMode::Windowed
                                        }
                                    }
                                }
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
