use crate::{app, page::*};
use bevy_ui_navigation::{prelude::*, NavRequestSystem};

const PAGE_CODE: &str = "auto";
const PAGE_NAME: &str = "Auto";
const PAGE_ICON: &str = "arrow-left-light";

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
        PageState::Auto
    }
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state()), page_enter)
            .add_systems(
                Update,
                handle_ui_navigation
                    .after(NavRequestSystem)
                    .run_if(in_state(self.state())),
            )
            .add_systems(OnExit(self.state()), app::ui::despawn_ui::<OnPage>);
    }
}

#[derive(Component)]
struct OnPage;

#[derive(Component)]
enum ButtonAction {
    BackToMainMenu,
}

fn page_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    padding: UiRect::all(app::ui::px_p(PAGE_PADDING)),
                    ..default()
                },
                ..default()
            },
            OnPage,
        ))
        .with_children(|parent| {
            app::ui::build_icon_btn(
                parent,
                &asset_server,
                (
                    ButtonAction::BackToMainMenu,
                    app::interaction::IaButton,
                    Focusable::new().prioritized(),
                ),
                Style {
                    position_type: PositionType::Absolute,
                    bottom: app::ui::px_p(PAGE_PADDING),
                    left: app::ui::px_p(PAGE_PADDING),
                    ..default()
                },
                "arrow-left-light",
            );
        });
}

fn handle_ui_navigation(
    mut actions: Query<&mut ButtonAction>,
    mut events: EventReader<NavEvent>,
    mut page_state: ResMut<NextState<PageState>>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut actions,
        |mut action| match &mut *action {
            ButtonAction::BackToMainMenu => page_state.set(PageState::Menu),
        },
    );
}
