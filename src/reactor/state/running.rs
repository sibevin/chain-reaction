use crate::{
    app,
    reactor::{self, field, hit::*, particle::*},
};
use bevy::{input, prelude::*};
use std::f32::consts::PI;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(reactor::ReactorState::Running), (state_setup,))
            .add_systems(
                Update,
                (
                    handle_pause_btn,
                    control_u_by_mouse,
                    control_u_by_keyboard,
                    pause_by_keyboard,
                    move_particle,
                    reactor::field::timer::update_field,
                    reactor::field::alpha_count::update_field,
                    reactor::field::score::update_field,
                    handle_particle_reaction,
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
                        ..default()
                    },
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
                    ParticleType::Alpha => {
                        particle.tick_countdown();
                    }
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

fn control_u_by_mouse(
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

const HYPER_HIT_BASE_SCORE: u32 = 100;
const CONTROL_HIT_SCORE: u32 = 100;

fn handle_particle_reaction(
    mut commands: Commands,
    mut particle_query: Query<(Entity, &mut Particle), With<Particle>>,
    mut timer_query: Query<&mut reactor::ReactorTimer>,
    time: Res<Time>,
    mut reactor_state: ResMut<NextState<reactor::ReactorState>>,
    settings: ResMut<app::settings::Settings>,
    audio_se_asset: Res<app::audio::AudioSeAsset>,
    mut field_score_query: Query<
        (&mut Text, &mut field::FieldScore),
        (With<field::FieldScore>, Without<field::FieldTimer>),
    >,
) {
    for mut timer in &mut timer_query {
        if timer.tick(time.delta()).just_finished() {
            let (mut text, mut field_score) = field_score_query.single_mut();
            let hit_map = detect_hit(&mut particle_query);
            let mut killed_entities: Vec<Entity> = vec![];
            for (e, mut p) in particle_query.iter_mut() {
                if let Some(action) = hit_map.get(&e) {
                    match p.particle_type() {
                        ParticleType::Alpha => match action {
                            HitAction::Kill => {
                                killed_entities.push(e);
                                app::audio::play_se(
                                    app::audio::AudioSe::Pop,
                                    &mut commands,
                                    &audio_se_asset,
                                    settings.as_ref(),
                                );
                            }
                            HitAction::Release(count) => {
                                p.reset_countdown();
                                if *count > 1 {
                                    for i in 1..=*count {
                                        let angle =
                                            PI * 2.0 * ((i - 1) as f32 + 0.25) / *count as f32;
                                        let direction = Vec2::new(angle.cos(), angle.sin());
                                        alpha::build_particle_sprite(
                                            &mut commands,
                                            reactor::RunningParticle,
                                            Some(p.pos() + direction * p.radius() * 3.0),
                                            Some(Particle::gen_random_v(Some(direction))),
                                            Some(1),
                                        );
                                    }
                                } else {
                                    let direction = Particle::gen_random_v(None).normalize();
                                    alpha::build_particle_sprite(
                                        &mut commands,
                                        reactor::RunningParticle,
                                        Some(p.pos() + direction * p.radius() * 3.0),
                                        None,
                                        None,
                                    );
                                }
                                app::audio::play_se(
                                    app::audio::AudioSe::Hit,
                                    &mut commands,
                                    &audio_se_asset,
                                    settings.as_ref(),
                                );
                                killed_entities.push(e);
                            }
                            HitAction::MoveOnly => {
                                p.reset_countdown();
                                p.set_v(Particle::gen_random_v(None))
                            }
                            _ => (),
                        },
                        ParticleType::Control => match action {
                            HitAction::AlphaHit(count) => {
                                for _ in 1..=*count {
                                    if p.tick_countdown() == 0 {
                                        killed_entities.push(e);
                                    }
                                    control::update_particle_sprite(
                                        &mut commands,
                                        e,
                                        p.level_ratio(),
                                        p.countdown_ratio(),
                                    );
                                }
                            }
                            HitAction::Kill => {
                                app::audio::play_se(
                                    app::audio::AudioSe::Pop,
                                    &mut commands,
                                    &audio_se_asset,
                                    settings.as_ref(),
                                );
                                killed_entities.push(e);
                            }
                            HitAction::UouHit => {
                                let radius = p.radius();
                                control::build_particle_sprite(
                                    &mut commands,
                                    reactor::RunningParticle,
                                    Some(field::gen_random_pos_in_field(radius)),
                                    None,
                                    Some(p.level() + 1),
                                );
                                p.update_level(1);
                                p.jump(field::gen_random_pos_in_field(radius));
                                p.reset_countdown();
                                app::audio::play_se(
                                    app::audio::AudioSe::PowerUp,
                                    &mut commands,
                                    &audio_se_asset,
                                    settings.as_ref(),
                                );
                                field_score.0 = field_score.0 + CONTROL_HIT_SCORE;
                                text.sections[0].value =
                                    field::format_field_text("score", field_score.0);
                            }
                            _ => (),
                        },
                        ParticleType::Hyper => match action {
                            HitAction::UouHit => {
                                control::build_particle_sprite(
                                    &mut commands,
                                    reactor::RunningParticle,
                                    None,
                                    None,
                                    Some(p.level()),
                                );
                                p.update_level(1);
                                let radius = p.radius();
                                p.jump(field::gen_random_pos_in_field(radius));
                                p.reset_countdown();
                                app::audio::play_se(
                                    app::audio::AudioSe::PowerUp,
                                    &mut commands,
                                    &audio_se_asset,
                                    settings.as_ref(),
                                );
                                field_score.0 =
                                    field_score.0 + HYPER_HIT_BASE_SCORE * p.level() as u32;
                                text.sections[0].value =
                                    field::format_field_text("score", field_score.0);
                            }
                            _ => (),
                        },
                        ParticleType::Uou => match action {
                            HitAction::Kill => {
                                reactor_state.set(reactor::ReactorState::Ended);
                                app::audio::play_se(
                                    app::audio::AudioSe::Boom,
                                    &mut commands,
                                    &audio_se_asset,
                                    settings.as_ref(),
                                );
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }
            }
            for entity in killed_entities {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

const KEYBOARD_DELTA_BIAS: f32 = 1.5;

fn control_u_by_keyboard(
    keyboard_input: Res<Input<KeyCode>>,
    mut u_particle_query: Query<(&mut Particle, &mut Transform), With<reactor::ControlParticle>>,
    settings: Res<app::settings::Settings>,
) {
    let (mut u_particle, mut u_transform) = u_particle_query.get_single_mut().unwrap();
    let mut delta: Vec2 = Vec2::default();
    if keyboard_input.pressed(KeyCode::W)
        || keyboard_input.pressed(KeyCode::Up)
        || keyboard_input.pressed(KeyCode::K)
    {
        delta.y = -KEYBOARD_DELTA_BIAS;
    }
    if keyboard_input.pressed(KeyCode::S)
        || keyboard_input.pressed(KeyCode::Down)
        || keyboard_input.pressed(KeyCode::J)
    {
        delta.y = KEYBOARD_DELTA_BIAS;
    }
    if keyboard_input.pressed(KeyCode::A)
        || keyboard_input.pressed(KeyCode::Left)
        || keyboard_input.pressed(KeyCode::H)
    {
        delta.x = -KEYBOARD_DELTA_BIAS;
    }
    if keyboard_input.pressed(KeyCode::D)
        || keyboard_input.pressed(KeyCode::Right)
        || keyboard_input.pressed(KeyCode::L)
    {
        delta.x = KEYBOARD_DELTA_BIAS;
    }
    let new_pos = calculate_u_new_pos(u_particle.pos(), delta, settings.get_value("sensitivity"));
    u_particle.jump(new_pos);
    u_transform.translation.x = new_pos.x;
    u_transform.translation.y = new_pos.y;
}

fn pause_by_keyboard(
    keyboard_input: Res<Input<KeyCode>>,
    mut reactor_state: ResMut<NextState<reactor::ReactorState>>,
) {
    if keyboard_input.pressed(KeyCode::Space)
        || keyboard_input.pressed(KeyCode::Return)
        || keyboard_input.pressed(KeyCode::Escape)
    {
        reactor_state.set(reactor::ReactorState::Paused)
    }
}
