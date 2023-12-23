use crate::{app, reactor};
use bevy::{app::AppExit, prelude::*};
use bevy_ui_navigation::{prelude::*, NavRequestSystem};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(reactor::ReactorState::Paused),
            (state_setup, app::audio::reduce_bgm_volume),
        )
        .add_systems(
            Update,
            handle_ui_navigation
                .after(NavRequestSystem)
                .run_if(in_state(reactor::ReactorState::Paused)),
        )
        .add_systems(
            OnExit(reactor::ReactorState::Paused),
            (app::audio::roll_bgm_volume_back, state_exit),
        );
    }
}

#[derive(Component)]
struct StateRootUi;

#[derive(Component)]
enum ButtonAction {
    Resume,
    Abort,
    Quit,
}

fn state_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: app::ui::COVER_COLOR.into(),
                ..default()
            },
            StateRootUi,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Paused",
                    TextStyle {
                        font: asset_server.load(app::ui::FONT),
                        font_size: app::ui::FONT_SIZE * 2.0,
                        color: app::ui::FG_COLOR,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(app::ui::px_p(10.0)),
                    ..default()
                }),
            );
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    app::ui::build_menu_entry(
                        parent,
                        &asset_server,
                        (
                            ButtonAction::Resume,
                            app::interaction::IaButton,
                            Focusable::default(),
                        ),
                        "Resume",
                        "play-light",
                    );
                    app::ui::build_menu_entry(
                        parent,
                        &asset_server,
                        (
                            ButtonAction::Abort,
                            app::interaction::IaButton,
                            Focusable::default(),
                        ),
                        "Abort",
                        "arrow-left-light",
                    );
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        app::ui::build_menu_entry(
                            parent,
                            &asset_server,
                            (
                                ButtonAction::Quit,
                                app::interaction::IaButton,
                                Focusable::default(),
                            ),
                            "Quit",
                            "sign-out-light",
                        );
                    }
                });
        });
}

fn state_exit(to_despawn: Query<Entity, With<StateRootUi>>, commands: Commands) {
    app::ui::despawn_ui::<StateRootUi>(to_despawn, commands);
}

fn handle_ui_navigation(
    mut actions: Query<&mut ButtonAction>,
    mut events: EventReader<NavEvent>,
    mut game_state: ResMut<NextState<app::GameState>>,
    mut reactor_state: ResMut<NextState<reactor::ReactorState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut actions,
        |mut action| match &mut *action {
            ButtonAction::Resume => reactor_state.set(reactor::ReactorState::Running),
            ButtonAction::Abort => game_state.set(app::GameState::Menu),
            ButtonAction::Quit => app_exit_events.send(AppExit),
        },
    );
}
