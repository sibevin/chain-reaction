use super::*;
use crate::app::ui;
pub struct State;

impl StudioStateBase for State {
    fn state(&self) -> StudioState {
        StudioState::Ready
    }
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state()), state_enter)
            .add_systems(Update, (state_running).run_if(in_state(self.state())))
            .add_systems(OnExit(self.state()), ui::despawn_ui::<OnState>);
    }
}

#[derive(Component)]
struct OnState;

fn state_enter(mut commands: Commands, asset_server: Res<AssetServer>) {}

fn state_running(mut commands: Commands, asset_server: Res<AssetServer>) {}
