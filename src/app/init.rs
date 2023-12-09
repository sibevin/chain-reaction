use crate::{app, reactor};
use bevy::{
    prelude::*,
    window::{Cursor, CursorIcon, PresentMode, WindowTheme},
};

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chain Reaction".into(),
                resolution: (app::WINDOW_W, app::WINDOW_H).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                cursor: Cursor {
                    icon: CursorIcon::Crosshair,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }));
    }
}

pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut particle_tmm: ResMut<reactor::tmm::ParticleTMM>,
) {
    commands.spawn(Camera2dBundle::default());
    reactor::field::build_reactor_field(&mut commands, &asset_server);
    reactor::tmm::init_particle_tmm(
        &mut particle_tmm,
        &asset_server,
        &mut meshes,
        &mut materials,
    )
}
