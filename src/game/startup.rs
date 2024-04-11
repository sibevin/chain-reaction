use super::*;
use crate::app::layer::{GAME_BG_Z_INDEX, GAME_COVER_Z_INDEX, GAME_FG_Z_INDEX};

#[derive(Component)]
pub struct GameFg;

#[derive(Component)]
pub struct GameBg;

#[derive(Component)]
pub struct GameDyn;

#[derive(Component)]
pub struct GameCover;

pub fn startup(commands: &mut Commands, game_status: &mut ResMut<GameStatus>) {
    game_status.switch_cover(true);
    game_status.is_refresh_required = true;
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, GAME_BG_Z_INDEX),
            ..default()
        },
        GameBg,
    ));
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, GAME_FG_Z_INDEX + 0.1),
            ..default()
        },
        GameFg,
    ));
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, GAME_FG_Z_INDEX + 0.2),
            ..default()
        },
        GameDyn,
    ));
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, GAME_COVER_Z_INDEX),
            ..default()
        },
        GameCover,
    ));
}
