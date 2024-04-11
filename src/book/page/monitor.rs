use super::*;
use crate::game;
use bevy_ui_navigation::{prelude::*, NavRequestSystem};

const PAGE_CODE: &str = "monitor";
const PAGE_NAME: &str = "";
const PAGE_ICON: &str = "";

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
        PageState::Monitor
    }
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(self.state()),
            (interaction::reset_default_focus, page_enter),
        )
        .add_systems(
            Update,
            (handle_ui_navigation, interaction::handle_default_focus)
                .after(NavRequestSystem)
                .run_if(in_state(self.state())),
        )
        .add_systems(
            OnExit(self.state()),
            (
                anime_effect::clear_anime_effect,
                page_exit,
                ui::despawn_ui::<OnPage>,
            ),
        );
    }
}

#[derive(Component)]
struct OnPage;

#[derive(Component)]
enum ButtonAction {
    MoveToPage(PageState),
}

fn page_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_status: ResMut<game::GameStatus>,
) {
    game_status.switch_cover(false);
    commands
        .spawn((build_page_layout(), OnPage))
        .with_children(|parent| {
            ui::build_icon_btn(
                parent,
                &asset_server,
                (
                    ButtonAction::MoveToPage(PageState::Menu),
                    app::interaction::IaButton,
                    Focusable::new().prioritized(),
                    app::interaction::IaDefaultFocus,
                ),
                Style {
                    position_type: PositionType::Absolute,
                    bottom: ui::px_p(ui::PAGE_PADDING),
                    left: ui::px_p(ui::PAGE_PADDING),
                    ..default()
                },
                "arrow-left-light_1.5x",
            );
        });
}

fn page_exit(mut game_status: ResMut<game::GameStatus>) {
    game_status.switch_cover(true);
}

fn handle_ui_navigation(
    mut actions: Query<&mut ButtonAction>,
    mut events: EventReader<NavEvent>,
    mut page_state: ResMut<NextState<PageState>>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut actions,
        |mut action| match &mut *action {
            ButtonAction::MoveToPage(state) => page_state.set(*state),
        },
    );
}
