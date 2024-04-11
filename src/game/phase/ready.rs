use super::*;
use crate::app::anime_effect;

pub struct Phase;

impl PhaseBase for Phase {
    fn state(&self) -> PhaseState {
        PhaseState::Demo
    }
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state()), state_enter)
            .add_systems(
                OnExit(self.state()),
                (anime_effect::clear_anime_effect, state_exit),
            );
    }
}

fn state_enter() {}

fn state_exit() {}
