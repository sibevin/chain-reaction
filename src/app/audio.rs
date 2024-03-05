use crate::app::settings;
use bevy::{audio::*, prelude::*};
use bevy_persistent::prelude::*;

#[derive(Component)]
pub struct AudioBgm;

pub enum AudioSe {
    Pop,
    PowerUp,
    Boom,
    Hit,
    Tada,
}

#[derive(Resource, Default)]
pub struct AudioSeAsset {
    pop: Option<Handle<AudioSource>>,
    power_up: Option<Handle<AudioSource>>,
    boom: Option<Handle<AudioSource>>,
    hit: Option<Handle<AudioSource>>,
    tada: Option<Handle<AudioSource>>,
}

impl AudioSeAsset {
    pub fn get(&self, se_type: AudioSe) -> &Option<Handle<AudioSource>> {
        match se_type {
            AudioSe::Pop => &self.pop,
            AudioSe::PowerUp => &self.power_up,
            AudioSe::Boom => &self.boom,
            AudioSe::Hit => &self.hit,
            AudioSe::Tada => &self.tada,
        }
    }
    pub fn set(&mut self, se_type: AudioSe, handle: Handle<AudioSource>) {
        match se_type {
            AudioSe::Pop => self.pop = Some(handle),
            AudioSe::PowerUp => self.power_up = Some(handle),
            AudioSe::Boom => self.boom = Some(handle),
            AudioSe::Hit => self.hit = Some(handle),
            AudioSe::Tada => self.tada = Some(handle),
        }
    }
}

pub struct AudioSeAssetPlugin;

impl Plugin for AudioSeAssetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AudioSeAsset::default());
    }
}

pub fn startup(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    audio_se_asset: &mut ResMut<AudioSeAsset>,
    settings: &Res<Persistent<settings::Settings>>,
) {
    init_se_asset(audio_se_asset, asset_server);
    build_bgm(commands, asset_server, settings);
}

fn init_se_asset(audio_se_asset: &mut ResMut<AudioSeAsset>, asset_server: &Res<AssetServer>) {
    audio_se_asset.set(AudioSe::Pop, asset_server.load("audio/se/erase.ogg"));
    audio_se_asset.set(AudioSe::PowerUp, asset_server.load("audio/se/pick_up.ogg"));
    audio_se_asset.set(AudioSe::Boom, asset_server.load("audio/se/game_over.ogg"));
    audio_se_asset.set(AudioSe::Hit, asset_server.load("audio/se/break_parts.ogg"));
    audio_se_asset.set(AudioSe::Tada, asset_server.load("audio/se/tada.ogg"));
}

fn build_bgm(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    settings: &Res<Persistent<settings::Settings>>,
) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/bgm/main.ogg"),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: Volume::Absolute(VolumeLevel::new(to_volume(settings.get_value("bgm")))),
                paused: !settings.is_enabled("bgm"),
                ..default()
            },
        },
        AudioBgm,
    ));
}

pub fn reduce_bgm_volume(
    settings: Res<Persistent<settings::Settings>>,
    audio_bgm_query: Query<&AudioSink, With<AudioBgm>>,
) {
    if let Ok(sink) = audio_bgm_query.get_single() {
        sink.set_volume(to_volume(settings.get_value("bgm")) / 4.0);
    }
}

pub fn roll_bgm_volume_back(
    settings: Res<Persistent<settings::Settings>>,
    audio_bgm_query: Query<&AudioSink, With<AudioBgm>>,
) {
    if let Ok(sink) = audio_bgm_query.get_single() {
        sink.set_volume(to_volume(settings.get_value("bgm")));
    }
}

const VOLUME_RATIO: f32 = 0.01;

pub fn to_volume(settings_value: u8) -> f32 {
    settings_value as f32 * VOLUME_RATIO
}

pub fn play_se(
    se_type: AudioSe,
    commands: &mut Commands,
    audio_se_asset: &Res<AudioSeAsset>,
    settings: &Persistent<settings::Settings>,
) {
    if settings.is_enabled("se") {
        commands.spawn((AudioBundle {
            source: audio_se_asset.get(se_type).as_ref().unwrap().clone(),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: Volume::Absolute(VolumeLevel::new(to_volume(settings.get_value("se")))),
                paused: false,
                ..default()
            },
        },));
    }
}
