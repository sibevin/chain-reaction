use crate::app::settings;
use bevy::{audio::*, prelude::*};
use bevy_persistent::prelude::*;

mod bgm;
mod se;
mod startup;

pub use bgm::reduce_bgm_volume;
pub use bgm::reset_bgm_volume;
pub use bgm::set_bgm_volume;
pub use bgm::QueryAudioBgm;
pub use se::play_se;
pub use startup::startup;

const VOLUME_RATIO: f32 = 0.01;

fn to_volume(settings_value: u8, volume_bias: f32) -> f32 {
    settings_value as f32 * VOLUME_RATIO + volume_bias
}
