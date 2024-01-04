use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_ui_navigation::prelude::*;

use chain_reaction::{app, page, reactor};

fn main() {
    App::new()
        .insert_resource(ClearColor(app::ui::BG_COLOR))
        .add_state::<app::GameState>()
        .add_systems(Startup, app::init::startup)
        .add_plugins((
            app::init::InitPlugin,
            ShapePlugin,
            DefaultNavigationPlugins,
            app::settings::SettingsPlugin,
            app::leaderboard::LeaderboardPlugin,
            app::audio::AudioSeAssetPlugin,
            app::key_binding::KeyBindingPlugin,
            reactor::ReactorPlugin,
            page::PagePlugin,
            app::interaction::InteractionPlugin,
        ))
        .run();
}
