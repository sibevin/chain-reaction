use super::*;
use crate::app::cursor;
use bevy::{ecs::schedule::SystemConfigs, input};

pub mod cross_panel;
pub mod menu_entry;
pub mod slider;
pub mod switcher;

#[derive(PartialEq, Eq)]
enum ElementPressAction {
    Confirm,
    Cancel,
}

#[derive(Default, Clone, Debug)]
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
}

#[derive(Component, Debug)]
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
}

#[derive(PartialEq)]
pub enum ElementAction {
    Confirm,
    Cancel,
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
        handle_element_mouse_clicking,
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
                _ => (),
            }
        }
    }
}

fn handle_element_mouse_clicking(
    mut ui_clicking_query: Query<
        (&Interaction, &GlobalTransform, &mut ElementData),
        Changed<Interaction>,
    >,
    window: Query<&Window>,
    cursor_data: Res<cursor::AppCursorData>,
    mut event_writer: EventWriter<ElementEvent>,
) {
    for (interaction, &g_trans, mut data) in ui_clicking_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match &mut data.as_mut() {
                ElementData::Switcher { ref mut data } => {
                    switcher::toggle_switcher(data);
                    event_writer.send(ElementEvent::DataChanged { data: data.clone() });
                }
                ElementData::Slider {
                    ref mut data,
                    is_modifier_on,
                    is_locked: _,
                } => {
                    slider::handle_mouse_clicking(
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
                    cross_panel::handle_mouse_clicking(
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
            }
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
            press_action = Some(ElementPressAction::Confirm);
        }
        if input.any_just_pressed([GamepadButton::new(gamepad, GamepadButtonType::East)]) {
            press_action = Some(ElementPressAction::Cancel);
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
        press_action = Some(ElementPressAction::Confirm);
    }
    if input.any_just_pressed([KeyCode::Escape, KeyCode::Delete]) {
        press_action = Some(ElementPressAction::Cancel);
    }
    if let Some(press_action) = press_action {
        handle_element_pressing(press_action, &mut ui_focusables, &mut event_writer);
    }
}

fn handle_element_pressing(
    press_action: ElementPressAction,
    ui_focusables: &mut Query<(Entity, &Focusable, &mut ElementData), With<ElementData>>,
    event_writer: &mut EventWriter<ElementEvent>,
) {
    for (entity, focus, mut data) in ui_focusables.iter_mut() {
        if matches!(focus.state(), FocusState::Focused) {
            let data = data.as_mut();
            match data {
                ElementData::Switcher { data } => {
                    if press_action == ElementPressAction::Confirm {
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

fn handle_element_gamepad_dpad_changing(
    gamepads: Res<Gamepads>,
    button_input: Res<Input<GamepadButton>>,
    mut ele_query: Query<&mut ElementData, With<ElementData>>,
    mut event_writer: EventWriter<ElementEvent>,
) {
    for gamepad in gamepads.iter() {
        let mut change: Option<(String, i8)> = None;
        if button_input
            .any_just_pressed([GamepadButton::new(gamepad, GamepadButtonType::DPadRight)])
        {
            change = Some((String::from("main"), 1));
        }
        if button_input.any_just_pressed([GamepadButton::new(gamepad, GamepadButtonType::DPadLeft)])
        {
            change = Some((String::from("main"), -1));
        }
        if button_input.any_just_pressed([GamepadButton::new(gamepad, GamepadButtonType::DPadUp)]) {
            change = Some((String::from("sub"), 1));
        }
        if button_input.any_just_pressed([GamepadButton::new(gamepad, GamepadButtonType::DPadDown)])
        {
            change = Some((String::from("sub"), -1));
        }
        if let Some(change) = change {
            handle_element_changing(change, &mut ele_query, &mut event_writer);
            return;
        }
    }
}
const GAMEPAD_MIN_THRESHOLD: f32 = 0.25;

fn handle_element_gamepad_axis_changing(
    gamepads: Res<Gamepads>,
    axis_input: Res<Axis<GamepadAxis>>,
    mut ele_query: Query<&mut ElementData, With<ElementData>>,
    mut event_writer: EventWriter<ElementEvent>,
    mut last_delta: Local<Vec2>,
    mut throttle_timer: ResMut<timer::ElementThrottleTimer>,
    time: Res<Time>,
) {
    if throttle_timer.0.tick(time.delta()).just_finished() {
        for gamepad in gamepads.iter() {
            let left_stick_x = axis_input
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
                .unwrap();
            if left_stick_x.abs() > GAMEPAD_MIN_THRESHOLD {
                last_delta.x = left_stick_x;
            } else {
                last_delta.x = 0.0;
            }
            let left_stick_y = axis_input
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
                .unwrap();
            if left_stick_y.abs() > GAMEPAD_MIN_THRESHOLD {
                last_delta.y = left_stick_y;
            } else {
                last_delta.y = 0.0;
            }
            if last_delta.x.abs() > 0.0 {
                let delta = if last_delta.x > 0.0 { 1 } else { -1 };
                let change = (String::from("main"), delta as i8);
                handle_element_changing(change, &mut ele_query, &mut event_writer);
            }
            if last_delta.y.abs() > 0.0 {
                let delta = if last_delta.y > 0.0 { 1 } else { -1 };
                let change = (String::from("sub"), delta as i8);
                handle_element_changing(change, &mut ele_query, &mut event_writer);
            }
        }
    }
}

fn handle_element_keyboard_changing(
    kb_input: Res<Input<KeyCode>>,
    mut ele_query: Query<&mut ElementData, With<ElementData>>,
    mut event_writer: EventWriter<ElementEvent>,
) {
    let mut change: Option<(String, i8)> = None;
    if kb_input.any_just_pressed([KeyCode::Right, KeyCode::D]) {
        change = Some((String::from("main"), 1));
    }
    if kb_input.any_just_pressed([KeyCode::Left, KeyCode::A]) {
        change = Some((String::from("main"), -1));
    }
    if kb_input.any_just_pressed([KeyCode::Up, KeyCode::W]) {
        change = Some((String::from("sub"), 1));
    }
    if kb_input.any_just_pressed([KeyCode::Down, KeyCode::S]) {
        change = Some((String::from("sub"), -1));
    }
    if let Some(change) = change {
        handle_element_changing(change, &mut ele_query, &mut event_writer);
    }
}

fn handle_element_changing(
    change: (String, i8),
    ele_query: &mut Query<&mut ElementData, With<ElementData>>,
    event_writer: &mut EventWriter<ElementEvent>,
) {
    for mut data in ele_query.iter_mut() {
        match &mut data.as_mut() {
            ElementData::Slider {
                ref mut data,
                is_modifier_on,
                is_locked,
            } => {
                if *is_locked {
                    let ori_value = data.u8_value.unwrap();
                    data.u8_value = Some(calculate_changed_value(
                        ori_value,
                        change.1,
                        *is_modifier_on,
                    ));
                    if ori_value != data.u8_value.unwrap() {
                        event_writer.send(ElementEvent::DataChanged { data: data.clone() });
                    }
                }
            }
            ElementData::CrossPanel {
                ref mut x,
                ref mut y,
                is_modifier_on,
                is_locked,
            } => {
                if *is_locked {
                    if change.0 == "main" {
                        let ori_x = x.u8_value.unwrap();
                        x.u8_value =
                            Some(calculate_changed_value(ori_x, change.1, *is_modifier_on));
                        if ori_x != x.u8_value.unwrap() {
                            event_writer.send(ElementEvent::DataChanged { data: x.clone() });
                        }
                    }
                    if change.0 == "sub" {
                        let ori_y = y.u8_value.unwrap();
                        y.u8_value =
                            Some(calculate_changed_value(ori_y, change.1, *is_modifier_on));
                        if ori_y != y.u8_value.unwrap() {
                            event_writer.send(ElementEvent::DataChanged { data: y.clone() });
                        }
                    }
                }
            }
            _ => (),
        }
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
