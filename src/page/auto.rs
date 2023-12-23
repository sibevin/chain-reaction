use bevy::prelude::*;
use bevy_ui_navigation::{prelude::*, NavRequestSystem};

use crate::{app, page};

pub struct PagePlugin;

impl Plugin for PagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(app::GameState::Auto), page_setup)
            .add_systems(
                Update,
                handle_ui_navigation
                    .after(NavRequestSystem)
                    .run_if(in_state(app::GameState::Auto)),
            )
            .add_systems(OnExit(app::GameState::Auto), app::ui::despawn_ui::<OnPage>);
    }
}

#[derive(Component)]
struct OnPage;

#[derive(Component)]
enum ButtonAction {
    BackToMainMenu,
}

fn page_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    padding: UiRect::all(app::ui::px_p(page::PAGE_PADDING)),
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
                    bottom: app::ui::px_p(page::PAGE_PADDING),
                    left: app::ui::px_p(page::PAGE_PADDING),
                    ..default()
                },
                "arrow-left-light",
            );
        });
}

fn handle_ui_navigation(
    mut actions: Query<&mut ButtonAction>,
    mut events: EventReader<NavEvent>,
    mut game_state: ResMut<NextState<app::GameState>>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut actions,
        |mut action| match &mut *action {
            ButtonAction::BackToMainMenu => game_state.set(app::GameState::Menu),
        },
    );
}
