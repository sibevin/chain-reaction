use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_tweening::*;
use bevy_ui_navigation::prelude::*;

use chain_reaction::*;

fn main() {
    App::new()
        .add_plugins((
            app::AppPlugin,
            ShapePlugin,
            DefaultNavigationPlugins,
            TweeningPlugin,
            book::BookPlugin,
            studio::StudioPlugin,
        ))
        .add_systems(Startup, app::startup)
        .run();
}
