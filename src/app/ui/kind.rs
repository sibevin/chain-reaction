use crate::app::{layer::UI_CANVAS_Z_INDEX, ui::*};
use bevy::input;

pub mod cross_panel;
pub mod slider;

#[derive(Clone, Debug)]
pub struct AppUiTargetValuePair {
    pub target: String,
    pub value: u8,
}

pub enum AppUiInitParams {
    Slider {
        data: AppUiTargetValuePair,
    },
    CrossPanel {
        x: AppUiTargetValuePair,
        y: AppUiTargetValuePair,
    },
}

#[derive(Component, Debug)]
pub enum AppUiData {
    Slider {
        data: AppUiTargetValuePair,
        canvas_em: AppUiCanvasEntityMap,
        is_modifier_on: bool,
        is_locked: bool,
    },
    CrossPanel {
        x: AppUiTargetValuePair,
        y: AppUiTargetValuePair,
        canvas_em: AppUiCanvasEntityMap,
        is_modifier_on: bool,
        is_locked: bool,
    },
}

#[derive(PartialEq)]
pub enum AppUiAction {
    Confirm,
    Cancel,
}

#[derive(Event, Debug)]
pub enum AppUiEvent {
    DataChanged { data: AppUiTargetValuePair },
    Focused { entity: Entity },
    Lock { entity: Entity },
    Unlock,
}

#[derive(Component)]
pub struct AppUiText;

#[derive(Debug)]
pub struct AppUiCanvasEntityMap {
    pub root: Entity,
    pub fg: Entity,
    pub bg: Entity,
}

#[derive(Component)]
pub struct AppUiCanvas;

pub fn create_ui_canvas(commands: &mut Commands) -> AppUiCanvasEntityMap {
    let root_entity = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, UI_CANVAS_Z_INDEX),
                ..default()
            },
            AppUiCanvas,
        ))
        .id();
    let mut bg_entity = Entity::PLACEHOLDER;
    let mut fg_entity = Entity::PLACEHOLDER;
    if let Some(mut entity_commands) = commands.get_entity(root_entity) {
        entity_commands.with_children(|parent| {
            bg_entity = parent
                .spawn(SpatialBundle {
                    transform: Transform::from_xyz(0.0, 0.0, UI_CANVAS_Z_INDEX + 0.1),
                    ..default()
                })
                .id();
            fg_entity = parent
                .spawn(SpatialBundle {
                    transform: Transform::from_xyz(0.0, 0.0, UI_CANVAS_Z_INDEX + 0.2),
                    ..default()
                })
                .id();
        });
    }
    AppUiCanvasEntityMap {
        root: root_entity,
        bg: bg_entity,
        fg: fg_entity,
    }
}

pub fn build_ui(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    bundle: impl Bundle,
    canvas_em: AppUiCanvasEntityMap,
    params: AppUiInitParams,
) {
    match params {
        AppUiInitParams::Slider { data } => {
            slider::build_slider_ui(canvas_em, parent, asset_server, bundle, data);
        }
        AppUiInitParams::CrossPanel { x, y } => {
            cross_panel::build_cross_panel_ui(canvas_em, parent, bundle, x, y);
        }
    }
}

pub fn update_ui_value(
    ui_query: &mut Query<(Entity, &mut AppUiData), With<AppUiData>>,
    target_value: AppUiTargetValuePair,
) {
    for (_, mut data) in ui_query.iter_mut() {
        match *data {
            AppUiData::Slider {
                ref mut data,
                canvas_em: _,
                is_modifier_on: _,
                is_locked: _,
            } => {
                if data.target == target_value.target {
                    data.value = target_value.value;
                }
            }
            AppUiData::CrossPanel {
                ref mut x,
                ref mut y,
                canvas_em: _,
                is_modifier_on: _,
                is_locked: _,
            } => {
                if x.target == target_value.target {
                    x.value = target_value.value;
                }
                if y.target == target_value.target {
                    y.value = target_value.value;
                }
            }
        }
    }
}

pub fn refresh_ui_canvas(
    mut commands: Commands,
    ui_query: Query<(&GlobalTransform, Entity, &mut AppUiData), With<AppUiData>>,
    mut ui_text_query: Query<(&Parent, &mut Text), With<AppUiText>>,
    window: Query<&Window>,
    mut delay_timer: ResMut<timer::AppUiBuildTimer>,
    mut refresh_timer: ResMut<timer::AppUiRefreshTimer>,
    time: Res<Time>,
) {
    if delay_timer.0.tick(time.delta()).just_finished() {
        for (g_trans, _, data) in ui_query.iter() {
            match data {
                AppUiData::Slider {
                    data: _,
                    canvas_em,
                    is_modifier_on: _,
                    is_locked: _,
                } => slider::init_ui_display(&mut commands, &window, &g_trans, &canvas_em),
                AppUiData::CrossPanel {
                    x: _,
                    y: _,
                    canvas_em,
                    is_modifier_on: _,
                    is_locked: _,
                } => cross_panel::init_ui_display(&mut commands, &window, &g_trans, &canvas_em),
            }
        }
    }
    if refresh_timer.0.tick(time.delta()).just_finished() {
        for (g_trans, entity, data) in ui_query.iter() {
            for (parent, mut ui_text) in ui_text_query.iter_mut() {
                if parent.get() == entity {
                    match data {
                        AppUiData::Slider {
                            data,
                            canvas_em: _,
                            is_modifier_on: _,
                            is_locked: _,
                        } => slider::update_ui_text(&mut ui_text, &data),
                        _ => (),
                    }
                }
            }
            match data {
                AppUiData::Slider {
                    data,
                    canvas_em,
                    is_modifier_on,
                    is_locked,
                } => slider::update_ui_display(
                    &mut commands,
                    &window,
                    &g_trans,
                    &canvas_em,
                    &data,
                    &is_modifier_on,
                    &is_locked,
                ),
                AppUiData::CrossPanel {
                    x,
                    y,
                    canvas_em,
                    is_modifier_on,
                    is_locked,
                } => cross_panel::update_ui_display(
                    &mut commands,
                    &window,
                    &g_trans,
                    &canvas_em,
                    &x,
                    &y,
                    &is_modifier_on,
                    &is_locked,
                ),
            }
        }
    }
}

pub fn clear_ui_canvas(
    mut commands: Commands,
    ae_query: Query<Entity, With<AppUiCanvas>>,
    mut build_timer: ResMut<timer::AppUiBuildTimer>,
    mut refresh_timer: ResMut<timer::AppUiRefreshTimer>,
) {
    for ae_entity in ae_query.iter() {
        if let Some(entity_commands) = commands.get_entity(ae_entity) {
            entity_commands.despawn_recursive()
        }
    }
    build_timer.0.reset();
    refresh_timer.0.reset();
}

pub fn handle_ui_mouse_clicking(
    mut ui_clicking_query: Query<
        (&Interaction, &GlobalTransform, &mut AppUiData),
        Changed<Interaction>,
    >,
    window: Query<&Window>,
    cursor_data: Res<app::cursor::AppCursorData>,
    mut event_writer: EventWriter<AppUiEvent>,
) {
    for (interaction, &g_trans, mut data) in ui_clicking_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match &mut data.as_mut() {
                AppUiData::Slider {
                    ref mut data,
                    canvas_em: _,
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
                    event_writer.send(AppUiEvent::DataChanged { data: data.clone() });
                }
                AppUiData::CrossPanel {
                    ref mut x,
                    ref mut y,
                    canvas_em: _,
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
                    event_writer.send(AppUiEvent::DataChanged { data: x.clone() });
                    event_writer.send(AppUiEvent::DataChanged { data: y.clone() });
                }
            }
        }
    }
}

pub fn handle_ui_mouse_unlock(
    mut ui_query: Query<&mut AppUiData, With<AppUiData>>,
    mut event_writer: EventWriter<AppUiEvent>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if mouse_input.get_just_pressed().next().is_some() {
        for mut data in ui_query.iter_mut() {
            let is_ui_locked;
            let data = data.as_mut();
            match data {
                AppUiData::Slider {
                    data: _,
                    canvas_em: _,
                    is_modifier_on: _,
                    ref mut is_locked,
                } => {
                    is_ui_locked = is_locked;
                }
                AppUiData::CrossPanel {
                    x: _,
                    y: _,
                    canvas_em: _,
                    is_modifier_on: _,
                    ref mut is_locked,
                } => {
                    is_ui_locked = is_locked;
                }
            };
            if *is_ui_locked {
                *is_ui_locked = false;
                event_writer.send(AppUiEvent::Unlock);
            }
        }
    }
}

pub fn handle_ui_mouse_dragging(
    mut ui_query: Query<(&Interaction, &mut AppUiData), With<AppUiData>>,
    mut motion_events: EventReader<input::mouse::MouseMotion>,
    mut event_writer: EventWriter<AppUiEvent>,
) {
    for (interaction, mut data) in ui_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match &mut data.as_mut() {
                AppUiData::Slider {
                    ref mut data,
                    canvas_em: _,
                    is_modifier_on,
                    is_locked: _,
                } => {
                    let ori_value = data.value;
                    slider::handle_mouse_dragging(&mut motion_events, data, is_modifier_on);
                    if ori_value != data.value {
                        event_writer.send(AppUiEvent::DataChanged { data: data.clone() });
                    }
                }
                AppUiData::CrossPanel {
                    ref mut x,
                    ref mut y,
                    canvas_em: _,
                    is_modifier_on,
                    is_locked: _,
                } => {
                    let ori_x = x.value;
                    let ori_y = y.value;
                    cross_panel::handle_mouse_dragging(&mut motion_events, x, y, is_modifier_on);
                    if ori_x != x.value {
                        event_writer.send(AppUiEvent::DataChanged { data: x.clone() });
                    }
                    if ori_y != y.value {
                        event_writer.send(AppUiEvent::DataChanged { data: y.clone() });
                    }
                }
            }
        }
    }
}

pub fn handle_ui_gamepad_lock(
    gamepads: Res<Gamepads>,
    input: Res<Input<GamepadButton>>,
    mut ui_focusables: Query<(Entity, &Focusable, &mut AppUiData), With<AppUiData>>,
    mut event_writer: EventWriter<AppUiEvent>,
) {
    for gamepad in gamepads.iter() {
        let mut is_locked_changed = None;
        if input.any_just_pressed([GamepadButton::new(gamepad, GamepadButtonType::South)]) {
            is_locked_changed = Some(true);
        }
        if input.any_just_pressed([GamepadButton::new(gamepad, GamepadButtonType::East)]) {
            is_locked_changed = Some(false);
        }
        if let Some(is_locked_value) = is_locked_changed {
            handle_ui_lock(is_locked_value, &mut ui_focusables, &mut event_writer);
        }
    }
}

pub fn handle_ui_keyboard_lock(
    input: Res<Input<KeyCode>>,
    mut ui_focusables: Query<(Entity, &Focusable, &mut AppUiData), With<AppUiData>>,
    mut event_writer: EventWriter<AppUiEvent>,
) {
    let mut is_locked_changed = None;
    if input.any_just_pressed([KeyCode::Space]) {
        is_locked_changed = Some(true);
    }
    if input.any_just_pressed([KeyCode::Escape, KeyCode::Delete]) {
        is_locked_changed = Some(false);
    }
    if let Some(is_locked_value) = is_locked_changed {
        handle_ui_lock(is_locked_value, &mut ui_focusables, &mut event_writer);
    }
}

fn handle_ui_lock(
    is_locked_value: bool,
    ui_focusables: &mut Query<(Entity, &Focusable, &mut AppUiData), With<AppUiData>>,
    event_writer: &mut EventWriter<AppUiEvent>,
) {
    for (entity, focus, mut data) in ui_focusables.iter_mut() {
        let is_ui_locked;
        let data = data.as_mut();
        match data {
            AppUiData::Slider {
                data: _,
                canvas_em: _,
                is_modifier_on: _,
                ref mut is_locked,
            } => {
                is_ui_locked = is_locked;
            }
            AppUiData::CrossPanel {
                x: _,
                y: _,
                canvas_em: _,
                is_modifier_on: _,
                ref mut is_locked,
            } => {
                is_ui_locked = is_locked;
            }
        };
        if matches!(focus.state(), FocusState::Focused) {
            if *is_ui_locked {
                if !is_locked_value {
                    *is_ui_locked = is_locked_value;
                    event_writer.send(AppUiEvent::Unlock);
                }
            } else {
                if is_locked_value {
                    *is_ui_locked = is_locked_value;
                    event_writer.send(AppUiEvent::Lock { entity });
                }
            }
        }
    }
}

pub fn handle_ui_gamepad_dpad_changing(
    gamepads: Res<Gamepads>,
    button_input: Res<Input<GamepadButton>>,
    mut ui_query: Query<&mut AppUiData, With<AppUiData>>,
    mut event_writer: EventWriter<AppUiEvent>,
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
            handle_ui_changing(change, &mut ui_query, &mut event_writer);
            return;
        }
    }
}
const GAMEPAD_MIN_THRESHOLD: f32 = 0.25;

pub fn handle_ui_gamepad_axis_changing(
    gamepads: Res<Gamepads>,
    axis_input: Res<Axis<GamepadAxis>>,
    mut ui_query: Query<&mut AppUiData, With<AppUiData>>,
    mut event_writer: EventWriter<AppUiEvent>,
    mut last_delta: Local<Vec2>,
    mut throttle_timer: ResMut<timer::AppUiThrottleTimer>,
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
                handle_ui_changing(change, &mut ui_query, &mut event_writer);
            }
            if last_delta.y.abs() > 0.0 {
                let delta = if last_delta.y > 0.0 { 1 } else { -1 };
                let change = (String::from("sub"), delta as i8);
                handle_ui_changing(change, &mut ui_query, &mut event_writer);
            }
        }
    }
}

pub fn handle_ui_keyboard_changing(
    kb_input: Res<Input<KeyCode>>,
    mut ui_query: Query<&mut AppUiData, With<AppUiData>>,
    mut event_writer: EventWriter<AppUiEvent>,
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
        handle_ui_changing(change, &mut ui_query, &mut event_writer);
    }
}

fn handle_ui_changing(
    change: (String, i8),
    ui_query: &mut Query<&mut AppUiData, With<AppUiData>>,
    event_writer: &mut EventWriter<AppUiEvent>,
) {
    for mut data in ui_query.iter_mut() {
        match &mut data.as_mut() {
            AppUiData::Slider {
                ref mut data,
                canvas_em: _,
                is_modifier_on,
                is_locked,
            } => {
                if *is_locked {
                    let ori_value = data.value;
                    data.value = calculate_changed_value(data.value, change.1, *is_modifier_on);
                    if ori_value != data.value {
                        event_writer.send(AppUiEvent::DataChanged { data: data.clone() });
                    }
                }
            }
            AppUiData::CrossPanel {
                ref mut x,
                ref mut y,
                canvas_em: _,
                is_modifier_on,
                is_locked,
            } => {
                if *is_locked {
                    if change.0 == "main" {
                        let ori_x = x.value;
                        x.value = calculate_changed_value(x.value, change.1, *is_modifier_on);
                        if ori_x != x.value {
                            event_writer.send(AppUiEvent::DataChanged { data: x.clone() });
                        }
                    }
                    if change.0 == "sub" {
                        let ori_y = y.value;
                        y.value = calculate_changed_value(y.value, change.1, *is_modifier_on);
                        if ori_y != y.value {
                            event_writer.send(AppUiEvent::DataChanged { data: y.clone() });
                        }
                    }
                }
            }
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

pub fn handle_ui_gamepad_modifier(
    gamepads: Res<Gamepads>,
    button_input: Res<Input<GamepadButton>>,
    mut ui_query: Query<&mut AppUiData, With<AppUiData>>,
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
            handle_ui_modifier(modifier_value, &mut ui_query);
        }
    }
}

pub fn handle_ui_keyboard_modifier(
    input: Res<Input<KeyCode>>,
    mut ui_query: Query<&mut AppUiData, With<AppUiData>>,
) {
    let mut modifier_changed_value: Option<bool> = None;
    if input.any_just_released([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        modifier_changed_value = Some(false);
    }
    if input.any_just_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        modifier_changed_value = Some(true);
    }
    if let Some(modifier_value) = modifier_changed_value {
        handle_ui_modifier(modifier_value, &mut ui_query);
    }
}

fn handle_ui_modifier(modifier_value: bool, ui_query: &mut Query<&mut AppUiData, With<AppUiData>>) {
    for mut data in ui_query.iter_mut() {
        match &mut data.as_mut() {
            AppUiData::Slider {
                data: _,
                canvas_em: _,
                ref mut is_modifier_on,
                is_locked: _,
            } => {
                *is_modifier_on = modifier_value;
            }
            AppUiData::CrossPanel {
                x: _,
                y: _,
                canvas_em: _,
                ref mut is_modifier_on,
                is_locked: _,
            } => {
                *is_modifier_on = modifier_value;
            }
        }
    }
}

pub fn apply_ui_lock(
    ui_entity: Option<Entity>,
    ui_query: &mut Query<(Entity, &mut AppUiData), With<AppUiData>>,
) {
    for (entity, mut data) in ui_query.iter_mut() {
        let is_ui_locked;
        let data = data.as_mut();
        match data {
            AppUiData::Slider {
                data: _,
                canvas_em: _,
                is_modifier_on: _,
                ref mut is_locked,
            } => {
                is_ui_locked = is_locked;
            }
            AppUiData::CrossPanel {
                x: _,
                y: _,
                canvas_em: _,
                is_modifier_on: _,
                ref mut is_locked,
            } => {
                is_ui_locked = is_locked;
            }
        };
        if let Some(ui_entity) = ui_entity {
            if ui_entity == entity {
                *is_ui_locked = true;
            } else {
                *is_ui_locked = false;
            }
        } else {
            *is_ui_locked = false;
        }
    }
}

pub fn to_canvas_pos(window: &Query<&Window>, window_pos: Vec2) -> Vec2 {
    let window = window.single();
    let win_w = window.resolution.width();
    let win_h = window.resolution.height();
    Vec2::new(window_pos.x, -window_pos.y) - Vec2::new(win_w / 2.0, -win_h / 2.0)
}

pub fn round_to_five(value: u8, enable: bool) -> u8 {
    if enable {
        (value as f32 / 5.0).round() as u8 * 5
    } else {
        value
    }
}
