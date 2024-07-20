use super::*;
use crate::app;
use bevy::utils::HashMap;
use std::path::Path;

pub struct AchievementPlugin;

impl Plugin for AchievementPlugin {
    fn build(&self, app: &mut App) {
        let config_dir = dirs::config_dir()
            .map(|native_config_dir| native_config_dir.join(app::APP_CODE))
            .unwrap_or(Path::new("local").join("configuration"));
        app.insert_resource(
            Persistent::<AchievementStore>::builder()
                .name("achievement")
                .format(StorageFormat::Bincode)
                .path(config_dir.join("achievement.bin"))
                .default(AchievementStore {
                    is_enabled: true,
                    pinned_codes: vec![],
                    record_map: HashMap::new(),
                })
                .build()
                .expect("failed to initialize variables"),
        );
        app.insert_resource(AchievementInfo::default());
    }
}
