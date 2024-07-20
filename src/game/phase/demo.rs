use super::*;
use crate::app::{anime_effect, ui};

pub struct Phase;

impl PhaseBase for Phase {
    fn state(&self) -> PhaseState {
        PhaseState::Demo
    }
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state()), state_enter)
            .add_systems(
                OnExit(self.state()),
                (
                    anime_effect::clear_anime_effect,
                    state_exit,
                    ui::despawn_ui::<OnPhase>,
                ),
            );
    }
}

#[derive(Component)]
struct OnPhase;

fn state_enter(mut game_status: ResMut<GameStatus>) {
    game_status.mode = StatusMode::Playing;
    game_status.increase("score", 10000);
    game_status.update("current_max_hyper_level", 3);
}

fn state_exit() {}
