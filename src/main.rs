use bevy::prelude::*;
use chain_reaction::{app, page, reactor};

fn main() {
    App::new()
        .insert_resource(ClearColor(app::ui::BG_COLOR))
        .add_state::<app::GameState>()
        .add_state::<reactor::ReactorState>()
        .add_systems(Startup, app::init::startup)
        .add_plugins((
            app::init::InitPlugin,
            app::settings::SettingsPlugin,
            page::menu::PagePlugin,
            page::game::PagePlugin,
            page::settings::PagePlugin,
            page::help::PagePlugin,
            page::about::PagePlugin,
            app::ui::ButtonInteractionPlugin,
        ))
        .run();
}
