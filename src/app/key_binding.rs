use bevy::prelude::*;
use bevy_ui_navigation::{
    events::Direction,
    prelude::{NavRequest, NavRequestSystem},
    systems::InputMapping,
};

#[derive(PartialEq, Default)]
pub enum KeyBindingMode {
    #[default]
    Navgation,
    Gaming,
    Keyboard,
}

#[derive(Resource, Default)]
pub struct KeyBindingConfig {
    pub mode: KeyBindingMode,
}

pub struct KeyBindingPlugin;

impl Plugin for KeyBindingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KeyBindingConfig {
            mode: KeyBindingMode::Navgation,
        })
        .add_systems(Startup, setup_input_mapping)
        .add_systems(Update, handle_key_binding.before(NavRequestSystem));
    }
}

fn setup_input_mapping(mut input_mapping: ResMut<InputMapping>) {
    input_mapping.keyboard_navigation = false;
    input_mapping.key_action = KeyCode::Return;
    input_mapping.focus_follows_mouse = true;
}

fn handle_key_binding(
    mut requests: EventWriter<NavRequest>,
    input: Res<Input<KeyCode>>,
    config: Res<KeyBindingConfig>,
) {
    move_by_arrow(&mut requests, &input);
    match config.mode {
        KeyBindingMode::Navgation => {
            if input.any_just_pressed([KeyCode::Space]) {
                requests.send(NavRequest::Action);
            }
            if input.any_just_pressed([KeyCode::Delete]) {
                requests.send(NavRequest::Cancel);
            }
            move_by_wsad(&mut requests, &input);
            move_by_kjhl(&mut requests, &input);
        }
        KeyBindingMode::Gaming => {
            if input.any_just_pressed([
                KeyCode::Space,
                KeyCode::Back,
                KeyCode::Delete,
                KeyCode::Escape,
            ]) {
                requests.send(NavRequest::Action);
            }
        }
        // NOTE: use default key binding only
        KeyBindingMode::Keyboard => (),
    }
}

fn move_by_arrow(requests: &mut EventWriter<NavRequest>, input: &Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Up) {
        requests.send(NavRequest::Move(Direction::North));
    }
    if input.just_pressed(KeyCode::Down) {
        requests.send(NavRequest::Move(Direction::South));
    }
    if input.just_pressed(KeyCode::Left) {
        requests.send(NavRequest::Move(Direction::West));
    }
    if input.just_pressed(KeyCode::Right) {
        requests.send(NavRequest::Move(Direction::East));
    }
}

fn move_by_wsad(requests: &mut EventWriter<NavRequest>, input: &Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::W) {
        requests.send(NavRequest::Move(Direction::North));
    }
    if input.just_pressed(KeyCode::S) {
        requests.send(NavRequest::Move(Direction::South));
    }
    if input.just_pressed(KeyCode::A) {
        requests.send(NavRequest::Move(Direction::West));
    }
    if input.just_pressed(KeyCode::D) {
        requests.send(NavRequest::Move(Direction::East));
    }
}

fn move_by_kjhl(requests: &mut EventWriter<NavRequest>, input: &Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::K) {
        requests.send(NavRequest::Move(Direction::North));
    }
    if input.just_pressed(KeyCode::J) {
        requests.send(NavRequest::Move(Direction::South));
    }
    if input.just_pressed(KeyCode::H) {
        requests.send(NavRequest::Move(Direction::West));
    }
    if input.just_pressed(KeyCode::L) {
        requests.send(NavRequest::Move(Direction::East));
    }
}
