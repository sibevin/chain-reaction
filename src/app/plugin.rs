use super::*;
use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    asset::AssetMetaCheck,
    prelude::*,
    window::{Cursor, PresentMode, WindowTheme},
};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(theme::BG_COLOR))
            .insert_resource(AssetMetaCheck::Never)
            .insert_resource(status::AppStatus::default())
            .add_plugins((
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        title: APP_NAME.into(),
                        resolution: (WINDOW_W, WINDOW_H).into(),
                        present_mode: PresentMode::AutoVsync,
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        window_theme: Some(WindowTheme::Dark),
                        enabled_buttons: bevy::window::EnabledButtons {
                            maximize: false,
                            ..default()
                        },
                        cursor: Cursor {
                            visible: false,
                            ..default()
                        },
                        ..default()
                    }),
                    ..default()
                }),
                AppSubPlugins,
            ));
    }
}

struct AppSubPlugins;

impl PluginGroup for AppSubPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(timer::TimerPlugin)
            .add(ui::AppUiPlugin)
            .add(settings::SettingsPlugin)
            .add(leaderboard::LeaderboardPlugin)
            .add(achievement::AchievementPlugin)
            .add(audio::AudioSeAssetPlugin)
            .add(key_binding::KeyBindingPlugin)
            .add(interaction::InteractionPlugin)
            .add(anime_effect::AnimeEffectPlugin)
            .add(cursor::AppCursorPlugin)
            .add(cursor_icon::AppCursorIconPlugin)
    }
}
