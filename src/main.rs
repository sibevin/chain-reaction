use bevy::prelude::*;
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
                reactor::timer::init_timer,
                reactor::field::score::init_field,
            ),
        )
        .add_systems(
            Update,
            (
                reactor::field::timer::update_field,
                reactor::field::alpha_count::update_field,
                reactor::field::score::update_field,
            ),
        )
        .add_plugins((
            app::init::InitPlugin,
            Shape2dPlugin::default(),
            app::settings::SettingsPlugin,
            reactor::state::demo::StatePlugin,
            page::menu::PagePlugin,
            page::game::PagePlugin,
            page::settings::PagePlugin,
            page::help::PagePlugin,
            page::about::PagePlugin,
            app::ui::ButtonInteractionPlugin,
        ))
        .run();
}
