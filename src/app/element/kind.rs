use super::*;
use crate::app::{cursor, cursor_icon};
use bevy::{ecs::schedule::SystemConfigs, input};

pub mod cross_panel;
pub mod menu_entry;
pub mod sensitivity_demo;
pub mod slider;
pub mod switcher;

#[derive(PartialEq, Eq)]
pub enum ElementAction {
    Confirm,
    Cancel,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct ElementTargetValuePair {
    pub target: String,
    pub u8_value: Option<u8>,
    pub bool_value: Option<bool>,
}

pub enum ElementInitParams {
    MenuEntry {
        icon: String,
        text: String,
    },
    Switcher {
        data: ElementTargetValuePair,
    },
    Slider {
        data: ElementTargetValuePair,
    },
    CrossPanel {
        x: ElementTargetValuePair,
        y: ElementTargetValuePair,
    },
    SensitivityDemo {
        default_sensitivity: ElementTargetValuePair,
        modified_sensitivity: ElementTargetValuePair,
    },
}

#[derive(Component, Debug, PartialEq)]
pub enum ElementData {
    MenuEntry,
    Switcher {
        data: ElementTargetValuePair,
    },
    Slider {
        data: ElementTargetValuePair,
        is_modifier_on: bool,
        is_locked: bool,
    },
    CrossPanel {
        x: ElementTargetValuePair,
        y: ElementTargetValuePair,
        is_modifier_on: bool,
        is_locked: bool,
    },
    SensitivityDemo {
        default_sensitivity: ElementTargetValuePair,
        modified_sensitivity: ElementTargetValuePair,
        ball_pos: Vec2,
        is_modifier_on: bool,
        is_locked: bool,
    },
}

#[derive(Event, Debug)]
pub enum ElementEvent {
    DataChanged { data: ElementTargetValuePair },
    Focused { entity: Entity },
    Lock { entity: Entity },
    Unlock,
}

#[derive(Component)]
pub struct ElementText;

#[derive(Component)]
pub struct ElementImage;

pub fn element_systems() -> SystemConfigs {
    (
        handle_element_mouse_unlock,
        handle_element_mouse_pressing,
        handle_element_mouse_dragging,
        handle_element_keyboard_pressing,
        handle_element_keyboard_changing,
        handle_element_gamepad_pressing,
        handle_element_gamepad_dpad_changing,
        handle_element_gamepad_axis_changing,
        handle_element_gamepad_modifier,
        handle_element_keyboard_modifier,
        refresh_elements,
    )
        .chain()
        .into_configs()
}

pub fn build_element(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    bundle: impl Bundle,
    params: ElementInitParams,
) {
    match params {
        ElementInitParams::MenuEntry { icon, text } => {
            menu_entry::build_element(parent, asset_server, bundle, icon.as_str(), text.as_str());
        }
        ElementInitParams::Switcher { data } => {
            switcher::build_element(parent, asset_server, bundle, data);
        }
        ElementInitParams::Slider { data } => {
            slider::build_element(parent, asset_server, bundle, data);
        }
        ElementInitParams::CrossPanel { x, y } => {
            cross_panel::build_element(parent, bundle, x, y);
        }
        ElementInitParams::SensitivityDemo {
            default_sensitivity,
            modified_sensitivity,
        } => {
            sensitivity_demo::build_element(
                parent,
                bundle,
                default_sensitivity,
                modified_sensitivity,
            );
        }
    }
}

pub fn update_element_value(
    ele_query: &mut Query<(Entity, &mut ElementData), With<ElementData>>,
    target_value: ElementTargetValuePair,
) {
    for (_, mut data) in ele_query.iter_mut() {
        match *data {
            ElementData::Switcher { ref mut data } => {
                if data.target == target_value.target && target_value.bool_value.is_some() {
                    data.bool_value = target_value.bool_value;
                }
            }
            ElementData::Slider {
                ref mut data,
                is_modifier_on: _,
                is_locked: _,
            } => {
                if data.target == target_value.target && target_value.u8_value.is_some() {
                    data.u8_value = target_value.u8_value;
                }
            }
            ElementData::CrossPanel {
                ref mut x,
                ref mut y,
                is_modifier_on: _,
                is_locked: _,
            } => {
                if target_value.u8_value.is_some() {
                    if x.target == target_value.target {
                        x.u8_value = target_value.u8_value;
                    }
                    if y.target == target_value.target {
                        y.u8_value = target_value.u8_value;
                    }
                }
            }
            ElementData::SensitivityDemo {
                ref mut default_sensitivity,
                ref mut modified_sensitivity,
                ball_pos: _,
                is_modifier_on: _,
                is_locked: _,
            } => {
                if target_value.u8_value.is_some() {
                    if default_sensitivity.target == target_value.target {
                        default_sensitivity.u8_value = target_value.u8_value;
                    }
                    if modified_sensitivity.target == target_value.target {
                        modified_sensitivity.u8_value = target_value.u8_value;
                    }
                }
            }
            _ => (),
        }
    }
}

pub fn clear_elements(
    mut commands: Commands,
    bg_query: Query<Entity, With<AppElementBg>>,
    fg_query: Query<Entity, With<AppElementFg>>,
    dyn_query: Query<Entity, With<AppElementDyn>>,
    mut build_timer: ResMut<timer::ElementBuildTimer>,
    mut refresh_timer: ResMut<timer::ElementRefreshTimer>,
) {
    let bg_entity = bg_query.get_single().unwrap();
    let fg_entity = fg_query.get_single().unwrap();
    let dyn_entity = dyn_query.get_single().unwrap();
    if let Some(mut entity_commands) = commands.get_entity(bg_entity) {
        entity_commands.despawn_descendants();
    }
    if let Some(mut entity_commands) = commands.get_entity(fg_entity) {
        entity_commands.despawn_descendants();
    }
    if let Some(mut entity_commands) = commands.get_entity(dyn_entity) {
        entity_commands.despawn_descendants();
    }
    build_timer.0.reset();
    refresh_timer.0.reset();
}

pub fn reset_elements(mut build_timer: ResMut<timer::ElementBuildTimer>) {
    build_timer.0.reset();
}

pub fn apply_element_lock(
    ele_entity: Option<Entity>,
    ele_query: &mut Query<(Entity, &mut ElementData), With<ElementData>>,
) {
    for (entity, mut data) in ele_query.iter_mut() {
        let is_ui_locked;
        let data = data.as_mut();
        match data {
            ElementData::Slider {
                data: _,
                is_modifier_on: _,
                ref mut is_locked,
            } => {
                is_ui_locked = is_locked;
            }
            ElementData::CrossPanel {
                x: _,
                y: _,
                is_modifier_on: _,
                ref mut is_locked,
            } => {
                is_ui_locked = is_locked;
            }
            ElementData::SensitivityDemo {
                default_sensitivity: _,
                modified_sensitivity: _,
                ball_pos: _,
                is_modifier_on: _,
                ref mut is_locked,
            } => {
                is_ui_locked = is_locked;
            }
            _ => continue,
        };
        if let Some(ele_entity) = ele_entity {
            if ele_entity == entity {
                *is_ui_locked = true;
            } else {
                *is_ui_locked = false;
            }
        } else {
            *is_ui_locked = false;
        }
    }
}

fn refresh_elements(
    mut commands: Commands,
    ele_query: Query<(&GlobalTransform, &Node, Entity, &mut ElementData), With<ElementData>>,
    bg_query: Query<Entity, With<AppElementBg>>,
    fg_query: Query<Entity, With<AppElementFg>>,
    dyn_query: Query<Entity, With<AppElementDyn>>,
    mut ui_text_query: Query<(&Parent, &mut Text), With<ElementText>>,
    mut ui_image_query: Query<(&Parent, &mut UiImage), With<ElementImage>>,
    window: Query<&Window>,
    mut delay_timer: ResMut<timer::ElementBuildTimer>,
    mut refresh_timer: ResMut<timer::ElementRefreshTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    let fg_entity = fg_query.get_single().unwrap();
    let bg_entity = bg_query.get_single().unwrap();
    let dyn_entity = dyn_query.get_single().unwrap();
    if delay_timer.0.tick(time.delta()).just_finished() {
        if let Some(mut entity_commands) = commands.get_entity(fg_entity) {
            entity_commands.despawn_descendants();
        }
        if let Some(mut entity_commands) = commands.get_entity(bg_entity) {
            entity_commands.despawn_descendants();
        }
        for (g_trans, node, _, ele_data) in ele_query.iter() {
            match ele_data {
                ElementData::MenuEntry => {
                    menu_entry::init_display(&mut commands, &window, &g_trans, &node, fg_entity)
                }
                ElementData::Slider {
                    data: _,
                    is_modifier_on: _,
                    is_locked: _,
                } => slider::init_display(&mut commands, &window, &g_trans, fg_entity),
                ElementData::CrossPanel {
                    x: _,
                    y: _,
                    is_modifier_on: _,
                    is_locked: _,
                } => cross_panel::init_display(&mut commands, &window, &g_trans, fg_entity),
                ElementData::SensitivityDemo {
                    default_sensitivity: _,
                    modified_sensitivity: _,
                    ball_pos: _,
                    is_modifier_on: _,
                    is_locked: _,
                } => sensitivity_demo::init_display(&mut commands, &window, &g_trans, fg_entity),
                _ => (),
            }
        }
    }
    if refresh_timer.0.tick(time.delta()).just_finished() {
        if let Some(mut entity_commands) = commands.get_entity(dyn_entity) {
            entity_commands.despawn_descendants();
        }
        for (g_trans, _, entity, ele_data) in ele_query.iter() {
            for (parent, mut ui_text) in ui_text_query.iter_mut() {
                if parent.get() == entity {
                    match ele_data {
                        ElementData::Slider {
                            data,
                            is_modifier_on: _,
                            is_locked: _,
                        } => slider::update_text(&mut ui_text, &data),
                        _ => (),
                    }
                }
            }
            for (parent, mut ui_image) in ui_image_query.iter_mut() {
                if parent.get() == entity {
                    match ele_data {
                        ElementData::Switcher { data } => {
                            switcher::update_ui_image(&mut ui_image, &data, &asset_server)
                        }
                        _ => (),
                    }
                }
            }
            match ele_data {
                ElementData::Slider {
                    data,
                    is_modifier_on,
                    is_locked,
                } => slider::update_display(
                    &mut commands,
                    &window,
                    &g_trans,
                    dyn_entity,
                    &data,
                    &is_modifier_on,
                    &is_locked,
                ),
                ElementData::CrossPanel {
                    x,
                    y,
                    is_modifier_on,
                    is_locked,
                } => cross_panel::update_display(
                    &mut commands,
                    &window,
                    &g_trans,
                    dyn_entity,
                    &x,
                    &y,
                    &is_modifier_on,
                    &is_locked,
                ),
                ElementData::SensitivityDemo {
                    default_sensitivity: _,
                    modified_sensitivity: _,
                    ball_pos,
                    is_modifier_on,
                    is_locked,
                } => sensitivity_demo::update_display(
                    &mut commands,
                    &window,
                    &g_trans,
                    dyn_entity,
                    &ball_pos,
                    &is_modifier_on,
                    &is_locked,
                ),
                _ => (),
            }
        }
    }
}

fn handle_element_mouse_pressing(
    mut ui_clicking_query: Query<
        (&Interaction, &GlobalTransform, &mut ElementData),
        Changed<Interaction>,
    >,
    window: Query<&Window>,
    cursor_data: Res<cursor::AppCursorData>,
    mut event_writer: EventWriter<ElementEvent>,
    mut cursor_icon_query: Query<(&mut UiImage, &mut cursor_icon::AppCursorIcon)>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, &g_trans, mut data) in ui_clicking_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => match &mut data.as_mut() {
                ElementData::Switcher { ref mut data } => {
                    switcher::toggle_switcher(data);
                    event_writer.send(ElementEvent::DataChanged { data: data.clone() });
                }
                ElementData::Slider {
                    ref mut data,
                    is_modifier_on,
                    is_locked: _,
                } => {
                    slider::handle_mouse_pressing(
                        &window,
                        &g_trans,
                        &cursor_data,
                        data,
                        is_modifier_on,
                    );
                    event_writer.send(ElementEvent::DataChanged { data: data.clone() });
                }
                ElementData::CrossPanel {
                    ref mut x,
                    ref mut y,
                    is_modifier_on,
                    is_locked: _,
                } => {
                    cross_panel::handle_mouse_pressing(
                        &window,
                        &g_trans,
                        &cursor_data,
                        x,
                        y,
                        is_modifier_on,
                    );
                    event_writer.send(ElementEvent::DataChanged { data: x.clone() });
                    event_writer.send(ElementEvent::DataChanged { data: y.clone() });
                }
                _ => (),
            },
            Interaction::Hovered => match &mut data.as_mut() {
                ElementData::SensitivityDemo {
                    default_sensitivity: _,
                    modified_sensitivity: _,
                    ball_pos: _,
                    is_modifier_on: _,
                    is_locked: _,
                } => {
                    cursor_icon::set_cursor_icon(&mut cursor_icon_query, &asset_server, "pointer");
                }
                _ => (),
            },
            Interaction::None => match &mut data.as_mut() {
                ElementData::SensitivityDemo {
                    default_sensitivity: _,
                    modified_sensitivity: _,
                    ball_pos: _,
                    is_modifier_on: _,
                    is_locked: _,
                } => {
                    cursor_icon::set_cursor_icon(&mut cursor_icon_query, &asset_server, "default");
                }
                _ => (),
            },
        }
    }
}

fn handle_element_mouse_unlock(
    mut ele_query: Query<&mut ElementData, With<ElementData>>,
    mut event_writer: EventWriter<ElementEvent>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if mouse_input.get_just_pressed().next().is_some() {
        for mut data in ele_query.iter_mut() {
            let is_ui_locked;
            let data = data.as_mut();
            match data {
                ElementData::Slider {
                    data: _,
                    is_modifier_on: _,
                    ref mut is_locked,
                } => {
                    is_ui_locked = is_locked;
                }
                ElementData::CrossPanel {
                    x: _,
                    y: _,
                    is_modifier_on: _,
                    ref mut is_locked,
                } => {
                    is_ui_locked = is_locked;
                }
                ElementData::SensitivityDemo {
                    default_sensitivity: _,
                    modified_sensitivity: _,
                    ball_pos: _,
                    is_modifier_on: _,
                    ref mut is_locked,
                } => {
                    is_ui_locked = is_locked;
                }
                _ => {
                    continue;
                }
            }
            if *is_ui_locked {
                *is_ui_locked = false;
                event_writer.send(ElementEvent::Unlock);
            }
        }
    }
}

fn handle_element_mouse_dragging(
    mut ele_query: Query<(&Interaction, &mut ElementData), With<ElementData>>,
    mut motion_events: EventReader<input::mouse::MouseMotion>,
    mut event_writer: EventWriter<ElementEvent>,
    mut cursor_icon_query: Query<(&mut UiImage, &mut cursor_icon::AppCursorIcon)>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut data) in ele_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match &mut data.as_mut() {
                ElementData::Switcher { ref mut data } => {
                    let ori_value = data.bool_value.unwrap();
                    switcher::handle_mouse_dragging(&mut motion_events, data);
                    let new_value = data.bool_value.unwrap();
                    if ori_value != new_value {
                        event_writer.send(ElementEvent::DataChanged { data: data.clone() });
                    }
                }
                ElementData::Slider {
                    ref mut data,
                    is_modifier_on,
                    is_locked: _,
                } => {
                    let ori_value = data.u8_value.unwrap();
                    slider::handle_mouse_dragging(&mut motion_events, data, is_modifier_on);
                    let new_value = data.u8_value.unwrap();
                    if ori_value != new_value {
                        event_writer.send(ElementEvent::DataChanged { data: data.clone() });
                    }
                }
                ElementData::CrossPanel {
                    ref mut x,
                    ref mut y,
                    is_modifier_on,
                    is_locked: _,
                } => {
                    let ori_x = x.u8_value.unwrap();
                    let ori_y = y.u8_value.unwrap();
                    cross_panel::handle_mouse_dragging(&mut motion_events, x, y, is_modifier_on);
                    if ori_x != x.u8_value.unwrap() {
                        event_writer.send(ElementEvent::DataChanged { data: x.clone() });
                    }
                    if ori_y != y.u8_value.unwrap() {
                        event_writer.send(ElementEvent::DataChanged { data: y.clone() });
                    }
                }
                ElementData::SensitivityDemo {
                    ref default_sensitivity,
                    ref modified_sensitivity,
                    ref mut ball_pos,
                    ref is_modifier_on,
                    is_locked: _,
                } => {
                    sensitivity_demo::handle_mouse_dragging(
                        &mut motion_events,
                        default_sensitivity,
                        modified_sensitivity,
                        &is_modifier_on,
                        ball_pos,
                    );
                    cursor_icon::set_cursor_icon(&mut cursor_icon_query, &asset_server, "move");
                }
                _ => (),
            }
        }
    }
}

fn handle_element_gamepad_pressing(
    gamepads: Res<Gamepads>,
    input: Res<Input<GamepadButton>>,
    mut ui_focusables: Query<(Entity, &Focusable, &mut ElementData), With<ElementData>>,
    mut event_writer: EventWriter<ElementEvent>,
) {
    for gamepad in gamepads.iter() {
        let mut press_action = None;
        if input.any_just_pressed([GamepadButton::new(gamepad, GamepadButtonType::South)]) {
            press_action = Some(ElementAction::Confirm);
        }
        if input.any_just_pressed([GamepadButton::new(gamepad, GamepadButtonType::East)]) {
            press_action = Some(ElementAction::Cancel);
        }
        if let Some(press_action) = press_action {
            handle_element_pressing(press_action, &mut ui_focusables, &mut event_writer);
        }
    }
}

fn handle_element_keyboard_pressing(
    input: Res<Input<KeyCode>>,
    mut ui_focusables: Query<(Entity, &Focusable, &mut ElementData), With<ElementData>>,
    mut event_writer: EventWriter<ElementEvent>,
) {
    let mut press_action = None;
    if input.any_just_pressed([KeyCode::Space, KeyCode::Return]) {
        press_action = Some(ElementAction::Confirm);
    }
    if input.any_just_pressed([KeyCode::Escape, KeyCode::Delete]) {
        press_action = Some(ElementAction::Cancel);
    }
    if let Some(press_action) = press_action {
        handle_element_pressing(press_action, &mut ui_focusables, &mut event_writer);
    }
}

fn handle_element_pressing(
    press_action: ElementAction,
    ui_focusables: &mut Query<(Entity, &Focusable, &mut ElementData), With<ElementData>>,
    event_writer: &mut EventWriter<ElementEvent>,
) {
    for (entity, focus, mut data) in ui_focusables.iter_mut() {
        if matches!(focus.state(), FocusState::Focused) {
            let data = data.as_mut();
            match data {
                ElementData::Switcher { data } => {
                    if press_action == ElementAction::Confirm {
                        switcher::toggle_switcher(data);
                        event_writer.send(ElementEvent::DataChanged { data: data.clone() });
                    }
                }
                ElementData::Slider {
                    data: _,
                    is_modifier_on: _,
                    ref mut is_locked,
                } => handle_element_locking(entity, is_locked, event_writer),
                ElementData::CrossPanel {
                    x: _,
                    y: _,
                    is_modifier_on: _,
                    ref mut is_locked,
                } => handle_element_locking(entity, is_locked, event_writer),
                ElementData::SensitivityDemo {
                    default_sensitivity: _,
                    modified_sensitivity: _,
                    ball_pos: _,
                    is_modifier_on: _,
                    ref mut is_locked,
                } => handle_element_locking(entity, is_locked, event_writer),
                _ => (),
            };
        }
    }
}

fn handle_element_locking(
    entity: Entity,
    current_is_locked: &mut bool,
    event_writer: &mut EventWriter<ElementEvent>,
) {
    if *current_is_locked {
        *current_is_locked = false;
        event_writer.send(ElementEvent::Unlock);
    } else {
        *current_is_locked = true;
        event_writer.send(ElementEvent::Lock { entity });
    }
}

const GAMEPAD_DPAD_CHANGING_DELTA: f32 = 0.1;

fn handle_element_gamepad_dpad_changing(
    gamepads: Res<Gamepads>,
    button_input: Res<Input<GamepadButton>>,
    mut ele_query: Query<&mut ElementData, With<ElementData>>,
    mut event_writer: EventWriter<ElementEvent>,
) {
    for gamepad in gamepads.iter() {
        let mut key_action: Option<ElementAction> = None;
        if button_input
            .any_just_pressed([GamepadButton::new(gamepad, GamepadButtonType::DPadRight)])
        {
            key_action = Some(ElementAction::Right);
        }

        if button_input.any_just_pressed([GamepadButton::new(gamepad, GamepadButtonType::DPadLeft)])
        {
            key_action = Some(ElementAction::Left);
        }

        if button_input.any_just_pressed([GamepadButton::new(gamepad, GamepadButtonType::DPadUp)]) {
            key_action = Some(ElementAction::Up);
        }

        if button_input.any_just_pressed([GamepadButton::new(gamepad, GamepadButtonType::DPadDown)])
        {
            key_action = Some(ElementAction::Down);
        }
        if let Some(key_action) = key_action {
            for mut data in ele_query.iter_mut() {
                handle_element_changing(
                    &key_action,
                    GAMEPAD_DPAD_CHANGING_DELTA,
                    &mut data,
                    &mut event_writer,
                    true,
                );
            }
        }
    }
}
const GAMEPAD_AXIS_MIN_THRESHOLD: f32 = 0.25;
const GAMEPAD_AXIS_CHANGING_DELTA: f32 = 2.0;

fn handle_element_gamepad_axis_changing(
    gamepads: Res<Gamepads>,
    axis_input: Res<Axis<GamepadAxis>>,
    mut ele_query: Query<&mut ElementData, With<ElementData>>,
    mut event_writer: EventWriter<ElementEvent>,
    mut last_delta: Local<Vec2>,
    mut throttle_timer: ResMut<timer::ElementThrottleTimer>,
    time: Res<Time>,
) {
    let mut is_triggered: bool = false;
    if throttle_timer.0.tick(time.delta()).just_finished() {
        is_triggered = true;
    }
    for gamepad in gamepads.iter() {
        let left_stick_x = axis_input
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        if left_stick_x.abs() > GAMEPAD_AXIS_MIN_THRESHOLD {
            last_delta.x = left_stick_x;
        } else {
            last_delta.x = 0.0;
        }
        let left_stick_y = axis_input
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();
        if left_stick_y.abs() > GAMEPAD_AXIS_MIN_THRESHOLD {
            last_delta.y = left_stick_y;
        } else {
            last_delta.y = 0.0;
        }
        if last_delta.x.abs() == 0.0 && last_delta.y.abs() == 0.0 {
            return;
        }
        for mut data in ele_query.iter_mut() {
            if last_delta.x.abs() > 0.0 {
                let action = if last_delta.x > 0.0 {
                    ElementAction::Right
                } else {
                    ElementAction::Left
                };
                handle_element_changing(
                    &action,
                    last_delta.x.abs() * GAMEPAD_AXIS_CHANGING_DELTA,
                    &mut data,
                    &mut event_writer,
                    is_triggered,
                );
            }
            if last_delta.y.abs() > 0.0 {
                let action = if last_delta.y > 0.0 {
                    ElementAction::Up
                } else {
                    ElementAction::Down
                };
                handle_element_changing(
                    &action,
                    last_delta.y.abs() * GAMEPAD_AXIS_CHANGING_DELTA,
                    &mut data,
                    &mut event_writer,
                    is_triggered,
                );
            }
        }
    }
}

const KEYBOARD_CHANGING_DELTA: f32 = 1.0;

fn handle_element_keyboard_changing(
    kb_input: Res<Input<KeyCode>>,
    mut ele_query: Query<&mut ElementData, With<ElementData>>,
    mut event_writer: EventWriter<ElementEvent>,
) {
    let mut key_actions: Vec<ElementAction> = vec![];
    let mut is_triggered: bool = false;
    if kb_input.any_pressed([KeyCode::Right, KeyCode::D]) {
        key_actions.push(ElementAction::Right);
    }
    if kb_input.any_pressed([KeyCode::Left, KeyCode::A]) {
        key_actions.push(ElementAction::Left);
    }
    if kb_input.any_pressed([KeyCode::Up, KeyCode::W]) {
        key_actions.push(ElementAction::Up);
    }
    if kb_input.any_pressed([KeyCode::Down, KeyCode::S]) {
        key_actions.push(ElementAction::Down);
    }
    if kb_input.any_just_pressed([KeyCode::Right, KeyCode::D]) {
        is_triggered = true;
    }
    if kb_input.any_just_pressed([KeyCode::Left, KeyCode::A]) {
        is_triggered = true;
    }
    if kb_input.any_just_pressed([KeyCode::Up, KeyCode::W]) {
        is_triggered = true;
    }
    if kb_input.any_just_pressed([KeyCode::Down, KeyCode::S]) {
        is_triggered = true;
    }
    if key_actions.len() == 0 {
        return;
    }
    for mut data in ele_query.iter_mut() {
        for key_action in key_actions.iter() {
            handle_element_changing(
                key_action,
                KEYBOARD_CHANGING_DELTA,
                &mut data,
                &mut event_writer,
                is_triggered,
            );
        }
    }
}

fn handle_element_changing(
    key_action: &ElementAction,
    delta: f32,
    ele_data: &mut ElementData,
    event_writer: &mut EventWriter<ElementEvent>,
    is_triggered: bool,
) {
    match ele_data {
        ElementData::Slider {
            ref mut data,
            ref is_modifier_on,
            ref is_locked,
        } => {
            if is_triggered {
                slider::handle_element_changing(
                    &key_action,
                    data,
                    is_modifier_on,
                    is_locked,
                    event_writer,
                );
            }
        }
        ElementData::CrossPanel {
            ref mut x,
            ref mut y,
            ref is_modifier_on,
            ref is_locked,
        } => {
            if is_triggered {
                cross_panel::handle_element_changing(
                    &key_action,
                    x,
                    y,
                    is_modifier_on,
                    is_locked,
                    event_writer,
                );
            }
        }
        ElementData::SensitivityDemo {
            ref default_sensitivity,
            ref modified_sensitivity,
            ref mut ball_pos,
            ref is_modifier_on,
            ref is_locked,
        } => {
            sensitivity_demo::handle_element_changing(
                &key_action,
                delta,
                default_sensitivity,
                modified_sensitivity,
                ball_pos,
                is_modifier_on,
                is_locked,
            );
        }
        _ => (),
    }
}

fn calculate_changed_value(ori_value: u8, delta: i8, is_modifier_on: bool) -> u8 {
    if is_modifier_on {
        round_to_five((ori_value as i8 + delta * 5).clamp(0, 100) as u8, true)
    } else {
        (ori_value as i8 + delta).clamp(0, 100) as u8
    }
}

fn handle_element_gamepad_modifier(
    gamepads: Res<Gamepads>,
    button_input: Res<Input<GamepadButton>>,
    mut ele_query: Query<&mut ElementData, With<ElementData>>,
) {
    for gamepad in gamepads.iter() {
        let mut modifier_changed_value: Option<bool> = None;
        if button_input.any_just_released([
            GamepadButton::new(gamepad, GamepadButtonType::RightTrigger),
            GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger),
        ]) {
            modifier_changed_value = Some(false);
        }
        if button_input.any_just_pressed([
            GamepadButton::new(gamepad, GamepadButtonType::RightTrigger),
            GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger),
        ]) {
            modifier_changed_value = Some(true);
        }
        if let Some(modifier_value) = modifier_changed_value {
            handle_element_modifier(modifier_value, &mut ele_query);
        }
    }
}

fn handle_element_keyboard_modifier(
    input: Res<Input<KeyCode>>,
    mut ele_query: Query<&mut ElementData, With<ElementData>>,
) {
    let mut modifier_changed_value: Option<bool> = None;
    if input.any_just_released([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        modifier_changed_value = Some(false);
    }
    if input.any_just_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        modifier_changed_value = Some(true);
    }
    if let Some(modifier_value) = modifier_changed_value {
        handle_element_modifier(modifier_value, &mut ele_query);
    }
}

fn handle_element_modifier(
    modifier_value: bool,
    ele_query: &mut Query<&mut ElementData, With<ElementData>>,
) {
    for mut data in ele_query.iter_mut() {
        match &mut data.as_mut() {
            ElementData::Slider {
                data: _,
                ref mut is_modifier_on,
                is_locked: _,
            } => {
                *is_modifier_on = modifier_value;
            }
            ElementData::CrossPanel {
                x: _,
                y: _,
                ref mut is_modifier_on,
                is_locked: _,
            } => {
                *is_modifier_on = modifier_value;
            }
            ElementData::SensitivityDemo {
                default_sensitivity: _,
                modified_sensitivity: _,
                ball_pos: _,
                ref mut is_modifier_on,
                is_locked: _,
            } => {
                *is_modifier_on = modifier_value;
            }
            _ => (),
        }
    }
}

fn to_canvas_pos(window: &Query<&Window>, window_pos: Vec2) -> Vec2 {
    let window = window.single();
    let win_w = window.resolution.width();
    let win_h = window.resolution.height();
    Vec2::new(window_pos.x, -window_pos.y) - Vec2::new(win_w / 2.0, -win_h / 2.0)
}

fn round_to_five(value: u8, enable: bool) -> u8 {
    if enable {
        (value as f32 / 5.0).round() as u8 * 5
    } else {
        value
    }
}
