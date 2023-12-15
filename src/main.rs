use bevy::prelude::*;
use bevy_ui_navigation::prelude::*;
use bevy_vector_shapes::prelude::*;
use chain_reaction::{app, page, reactor};

fn main() {
    App::new()
        .insert_resource(ClearColor(app::ui::BG_COLOR))
        .add_state::<app::GameState>()
        .add_state::<reactor::ReactorState>()
        .add_systems(
            Startup,
            (
                app::init::startup,
                app::audio::init_audio,
                reactor::timer::init_timer,
                reactor::field::score::init_field,
            ),
        )
        .add_plugins((
            app::init::InitPlugin,
            Shape2dPlugin::default(),
            DefaultNavigationPlugins,
            app::settings::SettingsPlugin,
            app::audio::AudioSeAssetPlugin,
            reactor::state::demo::StatePlugin,
            page::menu::PagePlugin,
            page::game::PagePlugin,
            page::settings::PagePlugin,
            page::help::PagePlugin,
            page::about::PagePlugin,
            app::interaction::InteractionPlugin,
        ))
        .run();
}
