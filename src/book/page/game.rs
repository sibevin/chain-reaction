use crate::{app::anime_effect, book::page::*, studio::*};

const PAGE_CODE: &str = "game";
const PAGE_NAME: &str = "Start";
const PAGE_ICON: &str = "play-light";

pub struct Page;

impl PageBase for Page {
    fn code(&self) -> &str {
        PAGE_CODE
    }
    fn name(&self) -> &str {
        PAGE_NAME
    }
    fn icon(&self) -> &str {
        PAGE_ICON
    }
    fn state(&self) -> PageState {
        PageState::Game
    }
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state()), page_enter)
            .add_systems(
                OnExit(self.state()),
                (anime_effect::clear_anime_effect, page_exit),
            );
    }
}

fn page_enter(mut studio_state: ResMut<NextState<StudioState>>) {
    studio_state.set(StudioState::Preparing);
}

fn page_exit(mut studio_state: ResMut<NextState<StudioState>>) {
    studio_state.set(StudioState::None);
}
