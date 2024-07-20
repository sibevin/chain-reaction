use super::model::LeaderboardModel;
use super::record::LeaderboardRecord;
use crate::app;
use bevy::prelude::*;
use bevy_persistent::prelude::*;
use std::path::Path;

pub struct LeaderboardPlugin;

impl Plugin for LeaderboardPlugin {
    fn build(&self, app: &mut App) {
        let config_dir = dirs::config_dir()
            .map(|native_config_dir| native_config_dir.join(app::APP_CODE))
            .unwrap_or(Path::new("local").join("configuration"));
        app.insert_resource(
            Persistent::<LeaderboardModel>::builder()
                .name("leaderboard")
                .format(StorageFormat::Bincode)
                .path(config_dir.join("leaderboard.bin"))
                .default(LeaderboardModel { records: vec![] })
                .build()
                .expect("failed to initialize variables"),
        );
        app.insert_resource(LeaderboardRecord::default());
    }
}
