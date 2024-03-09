use super::*;
use phf::phf_map;

struct AudioData {
    pub volume_bias: f32,
    pub path: &'static str,
}

static SE_MAP: phf::Map<&'static str, AudioData> = phf_map! {
    "erase" => AudioData {
        path: "audio/se/erase.ogg",
        volume_bias: 0.0,
    },
    "pick_up" => AudioData {
        path: "audio/se/pick_up.ogg",
        volume_bias: 0.0,
    },
    "game_over" => AudioData {
        path: "audio/se/game_over.ogg",
        volume_bias: 0.0,
    },
    "break" => AudioData {
        path: "audio/se/break.ogg",
        volume_bias: 0.0,
    },
    "tada" => AudioData {
        path: "audio/se/tada.ogg",
        volume_bias: 0.0,
    },
};

pub fn play_se(
    se: &str,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    settings: &Persistent<settings::Settings>,
) {
    if !settings.is_enabled("se") {
        return;
    }
    if let Some(audio_data) = SE_MAP.get(se) {
        commands.spawn((AudioBundle {
            source: asset_server.load(audio_data.path),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: Volume::Absolute(VolumeLevel::new(to_volume(
                    settings.get_value("se"),
                    audio_data.volume_bias,
                ))),
                paused: false,
                ..default()
            },
        },));
    }
}
