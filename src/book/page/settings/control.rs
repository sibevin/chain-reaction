use super::*;
use crate::app::{anime_effect, element, interaction, ui};
use bevy_mod_picking::prelude::*;
use bevy_persistent::prelude::*;
use bevy_ui_navigation::NavRequestSystem;

const PAGE_CODE: &str = "settings_control";
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
        PageState::SettingsControl
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
                handle_sensitivity_modifier,
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
                ui::despawn_ui::<DemoPanel>,
            ),
        );
    }
}

#[derive(Component)]
struct InteractionDefaultFocus;

#[derive(Component)]
struct OnPage;

#[derive(Component)]
struct DemoPanel;

#[derive(Component)]
struct DemoPanelUi;

#[derive(Component)]
struct DemoControlThumb;

#[derive(Component)]
struct DemoControlThumbSelection;

#[derive(Component)]
struct DemoControlCircle;

#[derive(Component)]
struct DemoInDragging;

fn page_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<Persistent<app::settings::Settings>>,
) {
    commands
        .spawn((build_page_layout(), OnPage, Pickable::IGNORE))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        ..default()
                    },
                    Pickable::IGNORE,
                ))
                .with_children(|parent| {
                    build_game_title(parent, &asset_server);
                    build_page_title(parent, &asset_server, PAGE_NAME, PAGE_ICON);
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    flex_grow: 1.0,
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            Pickable::IGNORE,
                        ))
                        .with_children(|parent| {
                            build_sep_title(parent, &asset_server, "Sensitivity", "gauge-fill");
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
                                    parent.spawn(TextBundle::from_section(
                                        "Default",
                                        TextStyle {
                                            font: asset_server.load(FONT),
                                            font_size: ui::FONT_SIZE,
                                            color: FG_COLOR,
                                        },
                                    ));
                                    element::build_element(
                                        parent,
                                        &asset_server,
                                        ButtonAction::AppUiNav,
                                        element::ElementInitParams::Slider {
                                            data: element::ElementTargetValuePair {
                                                target: String::from("sensitivity"),
                                                u8_value: Some(settings.get_value("sensitivity")),
                                                ..default()
                                            },
                                        },
                                    );
                                });
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
                                    let icon =
                                        asset_server.load("images/icons/arrow-fat-up-fill.png");
                                    parent.spawn(ImageBundle {
                                        style: Style {
                                            width: Val::Px(ui::ICON_SIZE),
                                            height: Val::Px(ui::ICON_SIZE),
                                            ..default()
                                        },
                                        image: UiImage::new(icon),
                                        ..default()
                                    });
                                    parent.spawn(TextBundle::from_section(
                                        "Shift",
                                        TextStyle {
                                            font: asset_server.load(FONT),
                                            font_size: ui::FONT_SIZE,
                                            color: FG_COLOR,
                                        },
                                    ));
                                    element::build_element(
                                        parent,
                                        &asset_server,
                                        ButtonAction::AppUiNav,
                                        element::ElementInitParams::Slider {
                                            data: element::ElementTargetValuePair {
                                                target: String::from("sensitivity_modified"),
                                                u8_value: Some(
                                                    settings.get_value("sensitivity_modified"),
                                                ),
                                                ..default()
                                            },
                                        },
                                    );
                                });
                            element::build_element(
                                parent,
                                &asset_server,
                                ButtonAction::AppUiNav,
                                element::ElementInitParams::SensitivityDemo {
                                    default_sensitivity: element::ElementTargetValuePair {
                                        target: String::from("sensitivity"),
                                        u8_value: Some(settings.get_value("sensitivity")),
                                        ..default()
                                    },
                                    modified_sensitivity: element::ElementTargetValuePair {
                                        target: String::from("sensitivity_modified"),
                                        u8_value: Some(settings.get_value("sensitivity_modified")),
                                        ..default()
                                    },
                                },
                            );
                        });
                    build_settings_nav_bar(parent, &asset_server, PageState::SettingsControl);
                });
        });
}

fn handle_sensitivity_modifier(
    input: Res<Input<KeyCode>>,
    mut status: ResMut<app::status::AppStatus>,
) {
    if input.any_just_released([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        status.in_modified_sensitivity = false;
    }
    if input.any_just_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        status.in_modified_sensitivity = true;
    }
}

fn handle_element_events(
    mut events: EventReader<element::ElementEvent>,
    mut settings: ResMut<Persistent<app::settings::Settings>>,
    mut ele_query: Query<(Entity, &mut element::ElementData), With<element::ElementData>>,
    mut nav_requests: EventWriter<NavRequest>,
) {
    for event in events.read() {
        match event {
            element::ElementEvent::DataChanged { data } => {
                if let Some(u8_value) = data.u8_value {
                    settings
                        .update(|settings| {
                            settings.set_value(data.target.as_str(), u8_value as i8);
                        })
                        .expect("failed to update slider");
                    element::update_element_value(&mut ele_query, data.clone());
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

fn handle_ui_navigation(
    action_query: Query<(Entity, &mut ButtonAction), With<ButtonAction>>,
    mut nav_events: EventReader<NavEvent>,
    mut page_state: ResMut<NextState<PageState>>,
    mut ele_query: Query<(Entity, &mut element::ElementData), With<element::ElementData>>,
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
                NavRequest::Unlock => {
                    element::apply_element_lock(None, &mut ele_query);
                }
                _ => (),
            },
            _ => (),
        }
    }
}
