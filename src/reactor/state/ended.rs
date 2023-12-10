use crate::{app, reactor};
use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(reactor::ReactorState::Ended),
            (state_setup, app::audio::reduce_bgm_volume),
        )
        .add_systems(
            Update,
            state_action.run_if(in_state(reactor::ReactorState::Ended)),
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
    BackToMenu,
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
                ..default()
            },
            StateRootUi,
        ))
        .with_children(|parent| {
            parent
                .spawn((NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Game Over",
                        TextStyle {
                            font: asset_server.load(app::ui::FONT),
                            font_size: app::ui::FONT_SIZE * 3.0,
                            color: Color::rgba(1.0, 0.0, 0.0, 0.8),
                            ..default()
                        },
                    ));
                });
            parent
                .spawn((NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Px(app::WINDOW_W),
                        height: Val::Px(app::WINDOW_H),
                        ..default()
                    },
                    ..default()
                },))
                .with_children(|parent| {
                    app::ui::build_icon_btn(
                        parent,
                        &asset_server,
                        ButtonAction::BackToMenu,
                        Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(18.0),
                            bottom: Val::Px(18.0),
                            ..default()
                        },
                        "arrow-left-light",
                    );
                });
        });
}

fn state_exit(to_despawn: Query<Entity, With<StateRootUi>>, commands: Commands) {
    app::ui::despawn_ui::<StateRootUi>(to_despawn, commands);
}

fn state_action(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<NextState<app::GameState>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                ButtonAction::BackToMenu => game_state.set(app::GameState::Menu),
            }
        }
    }
}
