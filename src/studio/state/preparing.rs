use super::*;
use crate::app::ui;
use crate::studio::StudioStatus;
pub struct State;

impl StudioStateBase for State {
    fn state(&self) -> StudioState {
        StudioState::Preparing
    }
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state()), state_enter)
            .add_systems(Update, (state_running).run_if(in_state(self.state())))
            .add_systems(OnExit(self.state()), ui::despawn_ui::<OnState>);
    }
}

#[derive(Component)]
struct OnState;

const UI_Z_INDEX: f32 = 2.0;
const BG_Z_INDEX: f32 = 1.0;

fn state_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Query<&Window>,
    mut status: ResMut<StudioStatus>,
) {
    status.reset();
    let window = window.single();
    let win_w = window.resolution.width();
    let win_h = window.resolution.height();
    commands.spawn(OnState).with_children(|parent| {
        parent.spawn((SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, UI_Z_INDEX),
            ..default()
        },));
        parent.spawn((SpriteBundle {
            texture: asset_server.load("pixel/bevy_pixel_dark.png"),
            transform: Transform::from_xyz(0., 0., BG_Z_INDEX),
            ..default()
        },));
    });
}

fn state_running(mut commands: Commands, asset_server: Res<AssetServer>) {}
