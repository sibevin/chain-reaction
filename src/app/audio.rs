use crate::app::settings;
use bevy::{audio::*, prelude::*};

#[derive(Component)]
pub struct AudioBgm;

pub enum AudioSe {
    Pop,
    PowerUp,
    Boom,
    Hit,
}

#[derive(Resource, Default)]
pub struct AudioSeAsset {
    pop: Option<Handle<AudioSource>>,
    power_up: Option<Handle<AudioSource>>,
    boom: Option<Handle<AudioSource>>,
    hit: Option<Handle<AudioSource>>,
}

impl AudioSeAsset {
    pub fn get(&self, se_type: AudioSe) -> &Option<Handle<AudioSource>> {
        match se_type {
            AudioSe::Pop => &self.pop,
            AudioSe::PowerUp => &self.power_up,
            AudioSe::Boom => &self.boom,
            AudioSe::Hit => &self.hit,
        }
    }
    pub fn set(&mut self, se_type: AudioSe, handle: Handle<AudioSource>) {
        match se_type {
            AudioSe::Pop => self.pop = Some(handle),
            AudioSe::PowerUp => self.power_up = Some(handle),
            AudioSe::Boom => self.boom = Some(handle),
            AudioSe::Hit => self.hit = Some(handle),
        }
    }
}

pub struct AudioSeAssetPlugin;

impl Plugin for AudioSeAssetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AudioSeAsset::default());
    }
}

pub fn init_audio_se_asset(
    audio_se_asset: &mut ResMut<AudioSeAsset>,
    asset_server: &Res<AssetServer>,
) {
    audio_se_asset.set(AudioSe::Pop, asset_server.load("audio/se/pick-92276.ogg"));
    audio_se_asset.set(
        AudioSe::PowerUp,
        asset_server.load("audio/se/item-pick-up-38258.ogg"),
    );
    audio_se_asset.set(
        AudioSe::Boom,
        asset_server.load("audio/se/heavy-cineamtic-hit-166888.ogg"),
    );
    audio_se_asset.set(
        AudioSe::Hit,
        asset_server.load("audio/se/glass-shatter-3-100155.ogg"),
    );
}

pub fn init_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load(
                "audio/bgm/synthetic-deception-loopable-epic-cyberpunk-crime-music-157454.ogg",
            ),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: Volume::Absolute(VolumeLevel::new(to_volume(50))),
                paused: false,
                ..default()
            },
            ..default()
        },
        AudioBgm,
    ));
}

pub fn reduce_bgm_volume(
    settings: Res<settings::Settings>,
    audio_bgm_query: Query<&AudioSink, With<AudioBgm>>,
) {
    if let Ok(sink) = audio_bgm_query.get_single() {
        sink.set_volume(to_volume(settings.get_value("bgm")) / 4.0);
    }
}

pub fn roll_bgm_volume_back(
    settings: Res<settings::Settings>,
    audio_bgm_query: Query<&AudioSink, With<AudioBgm>>,
) {
    if let Ok(sink) = audio_bgm_query.get_single() {
        sink.set_volume(to_volume(settings.get_value("bgm")));
    }
}

pub fn to_volume(settings_value: u8) -> f32 {
    settings_value as f32 * 0.02
}

pub fn play_se(
    se_type: AudioSe,
    commands: &mut Commands,
    audio_se_asset: &Res<AudioSeAsset>,
    settings: &settings::Settings,
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
            ..default()
        },));
    }
}
