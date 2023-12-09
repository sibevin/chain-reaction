use crate::{app, reactor};
use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(reactor::ReactorState::Running),
            (
                state_setup,
                reactor::field::timer::reset_field,
                reactor::field::score::reset_field,
            ),
        )
        .add_systems(
            Update,
            state_action.run_if(in_state(reactor::ReactorState::Running)),
        )
        .add_systems(OnExit(reactor::ReactorState::Running), state_exit);
    }
}

#[derive(Component)]
struct StateRootUi;

#[derive(Component)]
struct GameControlPanel;

#[derive(Component)]
enum ButtonAction {
    Pause,
}

fn state_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            StateRootUi,
        ))
        .with_children(|parent| {
            parent.spawn((
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
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    border_color: Color::RED.into(),
                    ..default()
                },
                GameControlPanel,
            ));
            parent
                .spawn((NodeBundle {
                    style: Style {
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
                        ButtonAction::Pause,
                        Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(18.0),
                            bottom: Val::Px(18.0),
                            ..default()
                        },
                        "pause-light",
                    );
                });
        });
}

fn state_exit(to_despawn: Query<Entity, With<StateRootUi>>, commands: Commands) {
    app::ui::despawn_ui::<StateRootUi>(to_despawn, commands);
}

fn state_action(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut reactor_state: ResMut<NextState<reactor::ReactorState>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                ButtonAction::Pause => reactor_state.set(reactor::ReactorState::Paused),
            }
        }
    }
}
