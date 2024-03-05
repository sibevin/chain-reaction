use super::*;
use bevy::{prelude::*, window::WindowMode};
use bevy_persistent::prelude::*;

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut audio_se_asset: ResMut<audio::AudioSeAsset>,
    settings: Res<Persistent<settings::Settings>>,
    mut window_query: Query<&mut Window>,
) {
    // fullscreen
    let mut window = window_query.single_mut();
    if settings.is_enabled("fullscreen") {
        window.mode = WindowMode::Fullscreen
    } else {
        window.mode = WindowMode::Windowed
    }

    // audio
    audio::startup(&mut commands, &asset_server, &mut audio_se_asset, &settings);

    // camera
    commands.spawn(Camera2dBundle::default());

    // cursor icon
    cursor_icon::init_cursor_icon(&mut commands, &asset_server);
}
