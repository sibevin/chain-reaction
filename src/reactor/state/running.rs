use crate::{
    app,
    reactor::{self, field, particle::*},
};
use bevy::{input, prelude::*};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(reactor::ReactorState::Running), (state_setup,))
            .add_systems(
                Update,
                (
                    handle_pause_btn,
                    control_u_particle,
                    move_particle,
                    reactor::field::timer::update_field,
                    reactor::field::alpha_count::update_field,
                    reactor::field::score::update_field,
                )
                    .run_if(in_state(reactor::ReactorState::Running)),
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
                Interaction::default(),
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

fn state_exit(commands: Commands, to_despawn: Query<Entity, With<StateRootUi>>) {
    app::ui::despawn_ui::<StateRootUi>(to_despawn, commands);
}

fn move_particle(
    mut commands: Commands,
    mut particle_query: Query<(Entity, &mut Transform, &mut Particle), With<Particle>>,
    mut timer_query: Query<&mut reactor::ReactorTimer>,
    time: Res<Time>,
    alpha_count_query: Query<&field::FieldAlphaCount, With<field::FieldAlphaCount>>,
) {
    for mut timer in &mut timer_query {
        if timer.tick(time.delta()).just_finished() {
            let alpha_count = alpha_count_query.single().0;
            for (entity, mut transform, mut particle) in particle_query.iter_mut() {
                if particle.particle_type() != ParticleType::Uou {
                    let new_pos = (*particle).travel();
                    transform.translation.x = new_pos.x;
                    transform.translation.y = new_pos.y;
                }
                match particle.particle_type() {
                    ParticleType::Hyper => {
                        if particle.level() > 1 {
                            if particle.tick_countdown() == 0 {
                                particle.update_level(-1);
                                particle.reset_countdown();
                            }
                        }
                        hyper::update_particle_sprite(
                            &mut commands,
                            entity,
                            particle.level_ratio(),
                            particle.countdown_ratio(),
                        );
                    }
                    ParticleType::Trigger => {
                        transform.rotate_z(-time.delta_seconds() * 2.0);
                        trigger::update_particle_level(particle.as_mut(), alpha_count);
                        if particle.tick_countdown() == 0 {
                            particle.reset_countdown();
                            let (_, _, angle) = transform.rotation.to_euler(EulerRot::XYZ);
                            let angle = angle + std::f32::consts::PI * 0.5;
                            let direction = Vec2::new(angle.cos(), angle.sin());
                            alpha::build_particle_sprite(
                                &mut commands,
                                reactor::RunningParticle,
                                Some(particle.pos() + direction * particle.radius()),
                                Some(Particle::gen_random_v(Some(direction))),
                                None,
                            );
                        }
                        trigger::update_particle_sprite(
                            &mut commands,
                            entity,
                            particle.countdown_ratio(),
                        );
                    }
                    _ => (),
                }
            }
        }
    }
}

fn control_u_particle(
    mut panel_query: Query<&Interaction, (With<Interaction>, With<GameControlPanel>)>,
    mut u_particle_query: Query<(&mut Particle, &mut Transform), With<reactor::ControlParticle>>,
    mut mouse_motion_events: EventReader<input::mouse::MouseMotion>,
    settings: Res<app::settings::Settings>,
) {
    for interaction in &mut panel_query {
        match *interaction {
            Interaction::Pressed => {
                for event in mouse_motion_events.read() {
                    let (mut u_particle, mut u_transform) =
                        u_particle_query.get_single_mut().unwrap();
                    let new_pos = calculate_u_new_pos(
                        u_particle.pos(),
                        event.delta,
                        settings.get_value("sensitivity"),
                    );
                    u_particle.jump(new_pos);
                    u_transform.translation.x = new_pos.x;
                    u_transform.translation.y = new_pos.y;
                }
            }
            _ => (),
        }
    }
}

fn handle_pause_btn(
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

fn calculate_u_new_pos(current: Vec2, delta: Vec2, sensitivity: u8) -> Vec2 {
    let delta_ratio = 0.5 + sensitivity as f32 / 100.0 * 5.0;
    let field_rect = field::get_field_rect(uou::RADIUS + 3.0);
    let new_x = (current.x + delta.x * delta_ratio).clamp(field_rect.min.x, field_rect.max.x);
    let new_y = (current.y - delta.y * delta_ratio).clamp(field_rect.min.y, field_rect.max.y);
    return Vec2::new(new_x, new_y);
}
