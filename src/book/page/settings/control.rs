use super::*;
use crate::{app::anime_effect, app::cursor_icon, app::interaction, app::ui};
use bevy_mod_picking::prelude::*;
use bevy_persistent::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_ui_navigation::NavRequestSystem;
use std::f32::consts::PI;

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
            (
                interaction::reset_default_focus,
                ui::clear_ui_canvas,
                page_enter,
            ),
        )
        .add_systems(
            Update,
            ((
                handle_ui_navigation,
                ui::handle_ui_mouse_unlock,
                ui::handle_ui_mouse_clicking,
                ui::handle_ui_mouse_dragging,
                ui::handle_ui_keyboard_lock,
                ui::handle_ui_keyboard_changing,
                ui::handle_ui_gamepad_lock,
                ui::handle_ui_gamepad_dpad_changing,
                ui::handle_ui_gamepad_axis_changing,
                interaction::handle_default_focus,
                ui::handle_ui_gamepad_modifier,
                ui::handle_ui_keyboard_modifier,
                ui::refresh_ui_canvas,
                handle_ui_events,
                handle_sensitivity_modifier,
            )
                .after(NavRequestSystem),)
                .run_if(in_state(self.state())),
        )
        .add_systems(
            OnExit(self.state()),
            (
                anime_effect::clear_anime_effect,
                ui::clear_ui_canvas,
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

const DEMO_PANEL_SIZE: f32 = 72.0;
const DEMO_PANEL_Z_INDEX: f32 = 1.0;
const DEMO_PANEL_R: f32 = ui::SPACE_SIZE * 30.0;
const DEMO_LINE_W: f32 = ui::SPACE_SIZE * 0.8;
const DEMO_CONTROL_R: f32 = ui::SPACE_SIZE * 5.0;
const DEMO_CIRCLE_DASH_SIZE: f32 = ui::SPACE_SIZE * 6.0;

fn page_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<Persistent<app::settings::Settings>>,
) {
    let s1_slider_canvas = ui::create_ui_canvas(&mut commands);
    let s2_slider_canvas = ui::create_ui_canvas(&mut commands);
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
                                    ui::build_ui(
                                        parent,
                                        &asset_server,
                                        ButtonAction::AppUiNav,
                                        s1_slider_canvas,
                                        ui::AppUiInitParams::Slider {
                                            data: ui::AppUiTargetValuePair {
                                                target: String::from("sensitivity"),
                                                value: settings.get_value("sensitivity"),
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
                                    ui::build_ui(
                                        parent,
                                        &asset_server,
                                        ButtonAction::AppUiNav,
                                        s2_slider_canvas,
                                        ui::AppUiInitParams::Slider {
                                            data: ui::AppUiTargetValuePair {
                                                target: String::from("sensitivity_modified"),
                                                value: settings.get_value("sensitivity_modified"),
                                            },
                                        },
                                    );
                                });
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            width: ui::px_p(DEMO_PANEL_SIZE),
                                            height: ui::px_p(DEMO_PANEL_SIZE),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Pickable::IGNORE,
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        NodeBundle {
                                            style: Style {
                                                width: ui::px_p(1.0),
                                                height: ui::px_p(1.0),
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        Pickable::IGNORE,
                                        DemoPanelUi,
                                        Focusable::default(),
                                    ));
                                });
                        });
                    build_settings_nav_bar(parent, &asset_server, PageState::SettingsControl);
                });
        });
    let mut circle_entity = Entity::PLACEHOLDER;
    let mut thumb_pos = (Vec2::default(), Vec2::default());
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0.0, -DEMO_PANEL_R, DEMO_PANEL_Z_INDEX),
                ..default()
            },
            DemoPanel,
        ))
        .with_children(|parent| {
            circle_entity = parent
                .spawn((
                    SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, DEMO_PANEL_Z_INDEX + 0.1),
                        ..default()
                    },
                    DemoControlCircle,
                    Pickable::IGNORE,
                ))
                .id();
            thumb_pos.0 = Vec2::from_angle(PI / 4.0) * DEMO_PANEL_R;
            build_demo_control_thumb(
                parent,
                &asset_server,
                Vec3::new(thumb_pos.0.x, thumb_pos.0.y, DEMO_PANEL_Z_INDEX + 0.2),
            );
            thumb_pos.1 = Vec2::from_angle(PI / 4.0 * 5.0) * DEMO_PANEL_R;
            build_demo_control_thumb(
                parent,
                &asset_server,
                Vec3::new(thumb_pos.1.x, thumb_pos.1.y, DEMO_PANEL_Z_INDEX + 0.3),
            );
        });

    draw_demo_circle(&mut commands, circle_entity, thumb_pos);
}

fn build_demo_control_thumb(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, pos: Vec3) {
    let circle = shapes::Circle {
        radius: DEMO_CONTROL_R,
        center: Vec2::ZERO,
    };
    let geo_builder = GeometryBuilder::new().add(&circle);
    parent
        .spawn((
            ShapeBundle {
                path: geo_builder.build(),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(pos.x, pos.y, pos.z),
                    ..default()
                },
                ..default()
            },
            Fill::color(Color::rgba(0.0, 0.0, 0.0, 0.0)),
            DemoControlThumb,
            PickableBundle::default(),
            On::<Pointer<DragStart>>::run(handle_demo_thumb_start_dragging),
            On::<Pointer<DragEnd>>::run(handle_demo_thumb_end_dragging),
            On::<Pointer<Drag>>::run(handle_demo_dragging),
            On::<Pointer<Over>>::run(handle_demo_thumb_over),
            On::<Pointer<Out>>::run(handle_demo_thumb_out),
        ))
        .with_children(|parent| {
            let geo_builder = GeometryBuilder::new().add(&circle);
            parent.spawn((
                ShapeBundle {
                    path: geo_builder.build(),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, pos.z + 0.01),
                        ..default()
                    },
                    ..default()
                },
                Stroke::new(theme::CONTROL_COLOR, DEMO_LINE_W * 0.5),
                Fill::color(theme::BG_COLOR.with_a(0.8)),
                Pickable::IGNORE,
            ));
            parent.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, pos.z + 0.02),
                    texture: asset_server.load("images/icons/plus-bold.png"),
                    ..default()
                },
                Pickable::IGNORE,
            ));
            parent.spawn((
                SpatialBundle {
                    transform: Transform::from_xyz(0.0, 0.0, pos.z + 0.03),
                    ..default()
                },
                DemoControlThumbSelection,
                Pickable::IGNORE,
            ));
        });
}

fn handle_demo_dragging(
    event: Listener<Pointer<Drag>>,
    mut thumb_query: Query<
        (Entity, &mut Transform, &Children),
        (With<DemoControlThumb>, Without<DemoControlCircle>),
    >,
    thumb_selection_query: Query<(Entity, &DemoControlThumbSelection)>,
    settings: Res<Persistent<app::settings::Settings>>,
    status: Res<app::status::AppStatus>,
    mut commands: Commands,
    circle_query: Query<Entity, (With<DemoControlCircle>, Without<DemoControlThumb>)>,
    mut cursor_icon_query: Query<(&mut UiImage, &mut cursor_icon::AppCursorIcon)>,
    asset_server: Res<AssetServer>,
) {
    let moving_ratio = if status.in_modified_sensitivity {
        settings.get_value("sensitivity_modified") as f32 / 50.0
    } else {
        settings.get_value("sensitivity") as f32 / 50.0
    };
    let mut thumb_trans: Vec<Vec2> = vec![];
    for (thumb_entity, mut transform, children) in thumb_query.iter_mut() {
        if thumb_entity == event.target {
            transform.translation.x += event.delta.x * moving_ratio;
            transform.translation.y -= event.delta.y * moving_ratio;
            for &child in children.iter() {
                if let Ok((entity, _)) = thumb_selection_query.get(child) {
                    draw_thumb_selection(&mut commands, entity, status.in_modified_sensitivity);
                }
            }
        }
        thumb_trans.push(Vec2::new(transform.translation.x, transform.translation.y));
    }
    let circle_entity = circle_query.single();
    let thumb_pos = (thumb_trans[0], thumb_trans[1]);
    draw_demo_circle(&mut commands, circle_entity, thumb_pos);
    cursor_icon::set_cursor_icon(&mut cursor_icon_query, &asset_server, "hidden");
}

fn handle_demo_thumb_start_dragging(
    event: Listener<Pointer<DragStart>>,
    mut thumb_query: Query<(Entity, &mut Pickable), With<DemoControlThumb>>,
    mut cursor_icon_query: Query<(&mut UiImage, &mut cursor_icon::AppCursorIcon)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut pickable) in thumb_query.iter_mut() {
        if entity == event.target {
            *pickable = Pickable::IGNORE;
            cursor_icon::set_cursor_icon(&mut cursor_icon_query, &asset_server, "hidden");
        }
    }
}

fn handle_demo_thumb_end_dragging(
    event: Listener<Pointer<DragEnd>>,
    mut commands: Commands,
    mut thumb_query: Query<(Entity, &mut Pickable), With<DemoControlThumb>>,
    thumb_selection_query: Query<
        (Entity, &Parent),
        (With<DemoControlThumbSelection>, Without<DemoControlThumb>),
    >,
    mut cursor_icon_query: Query<(&mut UiImage, &mut cursor_icon::AppCursorIcon)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut pickable) in thumb_query.iter_mut() {
        if entity == event.target {
            *pickable = Pickable::default();
            cursor_icon::set_cursor_icon(&mut cursor_icon_query, &asset_server, "default");
        }
    }
    for (entity, parent) in thumb_selection_query.iter() {
        if parent.get() == event.target {
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn_descendants();
                cursor_icon::set_cursor_icon(&mut cursor_icon_query, &asset_server, "default");
            }
        }
    }
}

fn handle_demo_thumb_over(
    event: Listener<Pointer<Over>>,
    mut commands: Commands,
    thumb_selection_query: Query<
        (Entity, &Parent),
        (With<DemoControlThumbSelection>, Without<DemoControlThumb>),
    >,
    demo_panel_ui_query: Query<Entity, With<DemoPanelUi>>,
    mut requests: EventWriter<NavRequest>,
    status: Res<app::status::AppStatus>,
    mut cursor_icon_query: Query<(&mut UiImage, &mut cursor_icon::AppCursorIcon)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, parent) in thumb_selection_query.iter() {
        if parent.get() == event.target {
            draw_thumb_selection(&mut commands, entity, status.in_modified_sensitivity);
            cursor_icon::set_cursor_icon(&mut cursor_icon_query, &asset_server, "pointer");
            if let Ok(dpu_entity) = demo_panel_ui_query.get_single() {
                requests.send(NavRequest::FocusOn(dpu_entity));
            }
        }
    }
}

fn draw_thumb_selection(commands: &mut Commands, thumb_entity: Entity, is_modified: bool) {
    if let Some(mut entity_commands) = commands.get_entity(thumb_entity) {
        let line_w_ratio = if is_modified { 3.0 } else { 2.0 };
        let circle = shapes::Circle {
            radius: DEMO_CONTROL_R * 1.5,
            center: Vec2::ZERO,
        };
        let geo_builder = GeometryBuilder::new().add(&circle);
        entity_commands.despawn_descendants();
        entity_commands.with_children(|parent| {
            parent.spawn((
                ShapeBundle {
                    path: geo_builder.build(),
                    ..default()
                },
                Stroke::new(theme::CONTROL_COLOR.with_a(0.8), DEMO_LINE_W * line_w_ratio),
                Pickable::IGNORE,
            ));
        });
    }
}

fn handle_demo_thumb_out(
    event: Listener<Pointer<Out>>,
    mut commands: Commands,
    thumb_selection_query: Query<
        (Entity, &Parent),
        (With<DemoControlThumbSelection>, Without<DemoControlThumb>),
    >,
    mut cursor_icon_query: Query<(&mut UiImage, &mut cursor_icon::AppCursorIcon)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, parent) in thumb_selection_query.iter() {
        if parent.get() == event.target {
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn_descendants();
                cursor_icon::set_cursor_icon(&mut cursor_icon_query, &asset_server, "default");
            }
        }
    }
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

fn draw_demo_circle(commands: &mut Commands, circle_entity: Entity, thumb_pos: (Vec2, Vec2)) {
    let center = (thumb_pos.0 + thumb_pos.1) / 2.0;
    let radius = thumb_pos.0.distance(thumb_pos.1) / 2.0;
    if let Some(mut entity_commands) = commands.get_entity(circle_entity) {
        entity_commands.despawn_descendants();
        entity_commands.with_children(|parent| {
            let circle = shapes::Circle { radius, center };
            let geo_builder = GeometryBuilder::new().add(&circle);
            parent.spawn((
                ShapeBundle {
                    path: geo_builder.build(),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, DEMO_PANEL_Z_INDEX + 0.11),
                        ..default()
                    },

                    ..default()
                },
                Stroke::new(theme::CONTROL_COLOR, DEMO_LINE_W),
                Pickable::IGNORE,
            ));
            let mut angle: f32 = 0.0;
            while angle < PI * 2.0 {
                angle += (DEMO_CIRCLE_DASH_SIZE / radius).atan();
                let dash_pos = Vec2::from_angle(angle) * radius;
                let circle = shapes::Circle {
                    radius: DEMO_LINE_W,
                    center,
                };
                let geo_builder = GeometryBuilder::new().add(&circle);
                parent.spawn((
                    ShapeBundle {
                        path: geo_builder.build(),
                        spatial: SpatialBundle {
                            transform: Transform::from_xyz(
                                dash_pos.x,
                                dash_pos.y,
                                DEMO_PANEL_Z_INDEX + 0.12,
                            ),
                            ..default()
                        },
                        ..default()
                    },
                    Fill::color(theme::BG_COLOR),
                    Pickable::IGNORE,
                ));
            }
            let geo_builder = GeometryBuilder::new().add(&circle);
            parent.spawn((
                ShapeBundle {
                    path: geo_builder.build(),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, DEMO_PANEL_Z_INDEX + 0.13),
                        ..default()
                    },

                    ..default()
                },
                Stroke::new(theme::CONTROL_COLOR.with_a(0.3), DEMO_LINE_W * 3.0),
                Pickable::IGNORE,
            ));
        });
    }
}

fn handle_ui_events(
    mut events: EventReader<ui::AppUiEvent>,
    mut settings: ResMut<Persistent<app::settings::Settings>>,
    mut ui_query: Query<(Entity, &mut ui::AppUiData), With<ui::AppUiData>>,
    mut nav_requests: EventWriter<NavRequest>,
) {
    for event in events.read() {
        match event {
            ui::AppUiEvent::DataChanged { data } => {
                settings
                    .update(|settings| {
                        settings.set_value(data.target.as_str(), data.value as i8);
                    })
                    .expect("failed to update slider");
                ui::update_ui_value(&mut ui_query, data.clone());
            }
            ui::AppUiEvent::Lock { entity: _ } => {
                nav_requests.send(NavRequest::Lock);
            }
            ui::AppUiEvent::Unlock => {
                nav_requests.send(NavRequest::Unlock);
            }
            _ => (),
        }
    }
}

fn handle_ui_navigation(
    action_query: Query<(Entity, &mut ButtonAction), With<ButtonAction>>,
    mut switch_btn_query: Query<(&Parent, &mut UiImage, &mut ui::SwitchButton)>,
    mut nav_events: EventReader<NavEvent>,
    mut page_state: ResMut<NextState<PageState>>,
    mut settings: ResMut<Persistent<app::settings::Settings>>,
    mut ui_query: Query<(Entity, &mut ui::AppUiData), With<ui::AppUiData>>,
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
                                }
                                ButtonAction::MoveToPage(state) => page_state.set(*state),
                                _ => (),
                            }
                        }
                    }
                }
                NavRequest::Unlock => {
                    ui::apply_ui_lock(None, &mut ui_query);
                }
                _ => (),
            },
            _ => (),
        }
    }
}
