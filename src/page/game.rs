use crate::{page::*, reactor};

const PAGE_CODE: &str = "game";
const PAGE_NAME: &str = "Start";
const PAGE_ICON: &str = "play-light";

pub struct PageDef;

impl PageDefBase for PageDef {
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
            .add_systems(OnExit(self.state()), page_exit);
    }
}

fn page_enter(mut reactor_state: ResMut<NextState<reactor::ReactorState>>) {
    reactor_state.set(reactor::ReactorState::Ready);
}

fn page_exit(
    mut commands: Commands,
    particle_query: Query<Entity, With<reactor::RunningParticle>>,
    mut reactor_state: ResMut<NextState<reactor::ReactorState>>,
) {
    for entity in &particle_query {
        commands.entity(entity).despawn_recursive();
    }
    reactor_state.set(reactor::ReactorState::Demo);
}
