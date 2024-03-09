use super::*;

#[derive(Component)]
pub struct AudioBgm;

pub type QueryAudioBgm<'w, 's> = Query<'w, 's, &'static AudioSink, With<AudioBgm>>;

const BGM_PATH: &str = "audio/bgm/main.ogg";
const BGM_VOLUME_BIAS: f32 = 0.0;

pub fn build_bgm(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    settings: &Res<Persistent<settings::Settings>>,
) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load(BGM_PATH),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: Volume::Absolute(VolumeLevel::new(to_volume(
                    settings.get_value("bgm"),
                    BGM_VOLUME_BIAS,
                ))),
                paused: !settings.is_enabled("bgm"),
                ..default()
            },
        },
        AudioBgm,
    ));
}

pub fn reduce_bgm_volume(
    settings: Res<Persistent<settings::Settings>>,
    audio_bgm_query: QueryAudioBgm,
) {
    set_bgm_volume(
        (settings.get_value("bgm") as f32 / 4.0) as u8,
        &audio_bgm_query,
    );
}

pub fn reset_bgm_volume(
    settings: Res<Persistent<settings::Settings>>,
    audio_bgm_query: QueryAudioBgm,
) {
    set_bgm_volume(settings.get_value("bgm"), &audio_bgm_query);
}

pub fn set_bgm_volume(volume: u8, audio_bgm_query: &QueryAudioBgm) {
    if let Ok(sink) = audio_bgm_query.get_single() {
        sink.set_volume(to_volume(volume, BGM_VOLUME_BIAS));
    }
}
