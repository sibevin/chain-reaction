use crate::{app, reactor};
use bevy::prelude::*;
use bevy_persistent::prelude::*;
use bevy_ui_navigation::{prelude::*, NavRequestSystem};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(reactor::ReactorState::Ended),
            (state_setup, app::audio::reduce_bgm_volume),
        )
        .add_systems(
            Update,
            handle_ui_navigation
                .after(NavRequestSystem)
                .run_if(in_state(reactor::ReactorState::Ended)),
        )
        .add_systems(
            OnExit(reactor::ReactorState::Ended),
            (app::audio::roll_bgm_volume_back, state_exit),
        );
    }
}

#[derive(Component)]
struct StateRootUi;

#[derive(Component)]
enum ButtonAction {
    ReStart,
    Leaderboard,
    BackToMenu,
}

const ENDED_BG_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.95);

fn state_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    status: ResMut<reactor::status::ReactorStatus>,
    leaderboard: Res<Persistent<app::leaderboard::Leaderboard>>,
    mut key_binding: ResMut<app::key_binding::KeyBindingConfig>,
) {
    key_binding.mode = app::key_binding::KeyBindingMode::Navgation;
    let lb_record = status.export();
    let is_new_record = leaderboard.is_new_record(&lb_record);
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
                background_color: ENDED_BG_COLOR.into(),
                ..default()
            },
            StateRootUi,
        ))
        .with_children(|parent| {
            parent
                .spawn((NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        row_gap: app::ui::px_p(8.0),
                        ..default()
                    },
                    ..default()
                },))
                .with_children(|parent| {
                    reactor::state::build_result_panel(
                        parent,
                        &asset_server,
                        &status,
                        &leaderboard,
                    );
                    if is_new_record {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    column_gap: app::ui::px_p(4.0),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: app::ui::px_p(90.0),
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            padding: UiRect::all(app::ui::px_p(3.0)),
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn(TextBundle::from_section(
                                            String::from(status.player_name.as_str()),
                                            TextStyle {
                                                font: asset_server.load(app::ui::FONT_DIGIT),
                                                font_size: app::ui::FONT_SIZE,
                                                color: app::ui::FG_COLOR,
                                                ..default()
                                            },
                                        ));
                                    });
                            });
                    }
                    parent
                        .spawn((NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                column_gap: app::ui::px_p(4.0),
                                ..default()
                            },
                            ..default()
                        },))
                        .with_children(|parent| {
                            app::ui::build_btn(
                                parent,
                                &asset_server,
                                (
                                    ButtonAction::ReStart,
                                    app::interaction::IaButton,
                                    Focusable::new().prioritized(),
                                ),
                                Style {
                                    padding: UiRect::all(app::ui::px_p(app::ui::BTN_PADDING)),
                                    ..default()
                                },
                                Some("Re-Start"),
                                Some("arrow-counter-clockwise"),
                            );
                            app::ui::build_btn(
                                parent,
                                &asset_server,
                                (
                                    ButtonAction::Leaderboard,
                                    app::interaction::IaButton,
                                    Focusable::default(),
                                ),
                                Style {
                                    padding: UiRect::all(app::ui::px_p(app::ui::BTN_PADDING)),
                                    ..default()
                                },
                                Some("Report"),
                                Some("list-numbers"),
                            );
                        });
                });
            app::ui::build_icon_btn(
                parent,
                &asset_server,
                (
                    ButtonAction::BackToMenu,
                    app::interaction::IaButton,
                    Focusable::default(),
                ),
                Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(18.0),
                    bottom: Val::Px(18.0),
                    ..default()
                },
                "arrow-left-light",
            );
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
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut actions,
        |mut action| match &mut *action {
            ButtonAction::BackToMenu => game_state.set(app::GameState::Menu),
            ButtonAction::ReStart => reactor_state.set(reactor::ReactorState::Ready),
            ButtonAction::Leaderboard => game_state.set(app::GameState::Leaderboard),
        },
    );
}
