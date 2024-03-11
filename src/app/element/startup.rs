use super::*;
use crate::app::layer::{ELEMENT_BG_Z_INDEX, ELEMENT_FG_Z_INDEX};

#[derive(Component)]
pub struct AppElementFg;

#[derive(Component)]
pub struct AppElementDyn;

#[derive(Component)]
pub struct AppElementBg;

pub fn startup(commands: &mut Commands) {
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, ELEMENT_BG_Z_INDEX),
            ..default()
        },
        AppElementBg,
    ));
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, ELEMENT_FG_Z_INDEX + 0.1),
            ..default()
        },
        AppElementFg,
    ));
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, ELEMENT_FG_Z_INDEX + 0.2),
            ..default()
        },
        AppElementDyn,
    ));
}
