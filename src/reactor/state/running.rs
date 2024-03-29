use crate::{
    app,
    reactor::{self, anime_effect::*, field, field_ach, hit::*, particle::*, status},
};
use bevy::{input, prelude::*};
#[cfg(not(target_arch = "wasm32"))]
use bevy::{render::view::window::screenshot::ScreenshotManager, window::PrimaryWindow};
use bevy_persistent::prelude::*;
use bevy_tweening::*;
use bevy_ui_navigation::{prelude::*, NavRequestSystem};
use std::collections::HashSet;
use std::f32::consts::PI;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(reactor::ReactorState::Running),
            (clear_anime_effect, state_setup),
        )
        .add_systems(
            Update,
            (
                detect_sensitivity_modification,
                control_u_by_mouse,
                control_u_by_keyboard,
                control_u_by_gamepad,
                handle_pause_btn.after(NavRequestSystem),
                move_particle,
                field::update_reactor_fields,
                field::update_target_fields,
                field_ach::update_ach_fields,
                handle_particle_reaction,
                component_animator_system::<Particle>,
                component_animator_system::<AnimeEffect>,
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

fn state_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut key_binding: ResMut<app::key_binding::KeyBindingConfig>,
) {
    key_binding.mode = app::key_binding::KeyBindingMode::Gaming;
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
                        (
                            ButtonAction::Pause,
                            app::interaction::IaButton,
                            Focusable::default(),
                        ),
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

fn state_exit(
    commands: Commands,
    to_despawn: Query<Entity, With<StateRootUi>>,
    status: Res<status::ReactorStatus>,
) {
    app::ui::despawn_ui::<StateRootUi>(to_despawn, commands);
    dbg!("(running) status = {}", status);
}

fn move_particle(
    mut commands: Commands,
    mut particle_query: Query<(&mut Transform, &mut Particle), With<Particle>>,
    mut reactor_timer: ResMut<reactor::ReactorTimer>,
    time: Res<Time>,
    status: Res<status::ReactorStatus>,
) {
    if reactor_timer.0.tick(time.delta()).just_finished() {
        let alpha_count = status.fetch("alpha_count");
        for (mut transform, mut particle) in particle_query.iter_mut() {
            if particle.is_traveling() {
                let new_pos = (*particle).travel();
                transform.translation.x = new_pos.x;
                transform.translation.y = new_pos.y;
                match particle.particle_type() {
                    ParticleType::Alpha => {
                        particle.tick_countdown();
                    }
                    ParticleType::Hyper => {
                        if particle.level() > 1 && particle.tick_countdown() == 0 {
                            particle.update_level(-1);
                            particle.reset_countdown();
                        }
                    }
                    ParticleType::Trigger => {
                        transform.rotate_z(-time.delta_seconds() * 2.0);
                        trigger::update_particle_level(particle.as_mut(), alpha_count);
                        if particle.tick_countdown() == 0 {
                            particle.reset_countdown();
                            let (_, _, angle) = transform.rotation.to_euler(EulerRot::XYZ);
                            let angle = angle + std::f32::consts::PI * 0.5;
                            let direction = Vec2::new(angle.cos(), angle.sin());
                            let level = alpha::pick_random_alpha_level();
                            alpha::build_particle_sprite(
                                &mut commands,
                                reactor::RunningParticle,
                                Some(particle.pos() + direction * particle.radius),
                                Some(direction),
                                Some(level),
                            );
                            if level > 2 {
                                insert_anime_effect(
                                    &mut commands,
                                    AnimeEffectParam {
                                        kind: AnimeEffectKind::Explosion,
                                        shape: AnimeEffectShape::Triangle,
                                        start_pos: particle.pos(),
                                        end_pos: particle.pos(),
                                    },
                                );
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}

fn handle_pause_btn(
    mut actions: Query<&mut ButtonAction>,
    mut events: EventReader<NavEvent>,
    mut reactor_state: ResMut<NextState<reactor::ReactorState>>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut actions,
        |mut action| match &mut *action {
            ButtonAction::Pause => reactor_state.set(reactor::ReactorState::Paused),
        },
    );
}

const HYPER_HIT_BASE_SCORE: u32 = 100;
const CONTROL_HIT_SCORE: u32 = 100;

#[allow(clippy::too_many_arguments)]
fn handle_particle_reaction(
    mut commands: Commands,
    mut particle_query: Query<(Entity, &mut Particle, &mut Transform), With<Particle>>,
    ae_query: Query<(Entity, &mut AnimeEffect), With<AnimeEffect>>,
    mut tween_completed_events: EventReader<TweenCompleted>,
    mut reactor_timer: ResMut<reactor::ReactorTimer>,
    mut painter_timer: ResMut<reactor::PainterTimer>,
    time: Res<Time>,
    mut reactor_state: ResMut<NextState<reactor::ReactorState>>,
    settings: Res<Persistent<app::settings::Settings>>,
    audio_se_asset: Res<app::audio::AudioSeAsset>,
    mut status: ResMut<status::ReactorStatus>,
    #[cfg(not(target_arch = "wasm32"))] main_window: Query<Entity, With<PrimaryWindow>>,
    #[cfg(not(target_arch = "wasm32"))] mut screenshot_manager: ResMut<ScreenshotManager>,
) {
    if painter_timer.0.tick(time.delta()).just_finished() {
        for (_, particle, _) in particle_query.iter() {
            particle.state_update(&mut commands);
        }
        for (_, mut particle, _) in particle_query.iter_mut() {
            if particle.state == ParticleState::Created {
                particle.state_setup(&mut commands);
            }
        }
        for (_, ae) in ae_query.iter() {
            update_anime_effect(&mut commands, ae);
        }
    }
    if reactor_timer.0.tick(time.delta()).just_finished() {
        let mut u_pos: Vec2 = Vec2::default();
        for (_, p, t) in particle_query.iter() {
            if p.particle_type() == ParticleType::Uou {
                u_pos = Vec2::new(t.translation.x, t.translation.y);
            }
        }
        status.update_stopping_time(u_pos);
        let hit_map = detect_hit(&mut particle_query);
        let mut entities_to_despawn: HashSet<Entity> = HashSet::new();
        for (e, mut p, mut t) in particle_query.iter_mut() {
            if let Some(action) = hit_map.get(&e) {
                match p.particle_type() {
                    ParticleType::Alpha => match action {
                        HitAction::Kill => {
                            alpha::setup_particle_ending(&mut commands, &mut p);
                            app::audio::play_se(
                                app::audio::AudioSe::Pop,
                                &mut commands,
                                &audio_se_asset,
                                &settings,
                            );
                        }
                        HitAction::Release(count) => {
                            p.reset_countdown();
                            if *count > 1 {
                                for i in 1..=*count {
                                    let angle = PI * 2.0 * ((i - 1) as f32 + 0.25) / *count as f32;
                                    let direction = Vec2::new(angle.cos(), angle.sin());
                                    alpha::build_particle_sprite(
                                        &mut commands,
                                        reactor::RunningParticle,
                                        Some(p.pos() + direction * p.radius * 3.0),
                                        Some(direction),
                                        Some(1),
                                    );
                                }
                            } else {
                                let direction = Particle::gen_random_direction();
                                alpha::build_particle_sprite(
                                    &mut commands,
                                    reactor::RunningParticle,
                                    Some(p.pos() + direction * p.radius * 3.0),
                                    None,
                                    None,
                                );
                            }
                            if *count > 3 {
                                insert_anime_effect(
                                    &mut commands,
                                    AnimeEffectParam {
                                        kind: AnimeEffectKind::Explosion,
                                        shape: AnimeEffectShape::Circle,
                                        start_pos: p.pos(),
                                        end_pos: p.pos(),
                                    },
                                );
                            }
                            app::audio::play_se(
                                app::audio::AudioSe::Hit,
                                &mut commands,
                                &audio_se_asset,
                                &settings,
                            );
                            entities_to_despawn.insert(e);
                        }
                        HitAction::MoveOnly => {
                            p.reset_countdown();
                            p.assign_random_v(None);
                        }
                        _ => (),
                    },
                    ParticleType::Control => match action {
                        HitAction::AlphaHit(count) => {
                            for _ in 1..=*count {
                                if p.tick_countdown() == 0 {
                                    control::setup_particle_ending(&mut commands, &mut p);
                                }
                            }
                        }
                        HitAction::Kill => {
                            control::setup_particle_ending(&mut commands, &mut p);
                        }
                        HitAction::UouHit => {
                            insert_anime_effect(
                                &mut commands,
                                AnimeEffectParam {
                                    kind: AnimeEffectKind::Explosion,
                                    shape: AnimeEffectShape::Square,
                                    start_pos: p.pos(),
                                    end_pos: p.pos(),
                                },
                            );
                            if let Some(pos) = status.prev_chain_pos(status::StatusChain::Control) {
                                insert_anime_effect(
                                    &mut commands,
                                    AnimeEffectParam {
                                        kind: AnimeEffectKind::Bullet,
                                        shape: AnimeEffectShape::Square,
                                        start_pos: pos,
                                        end_pos: p.pos(),
                                    },
                                );
                            };
                            let current_level = p.level();
                            let new_c_pos = field::gen_random_pos_in_field(p.radius);
                            control::build_particle_sprite(
                                &mut commands,
                                reactor::RunningParticle,
                                Some(new_c_pos),
                                Some(new_c_pos - u_pos),
                                Some(current_level + 1),
                            );
                            status.increase("total_control_count", 1);
                            p.update_level(1);
                            status.compare_and_update_max_field("control_level", p.level() as u32);
                            let ori_c_pos = field::gen_random_pos_in_field(p.radius);
                            p.jump(ori_c_pos);
                            t.translation.x = ori_c_pos.x;
                            t.translation.y = ori_c_pos.y;
                            p.assign_random_v(Some(ori_c_pos - u_pos));
                            app::audio::play_se(
                                app::audio::AudioSe::PowerUp,
                                &mut commands,
                                &audio_se_asset,
                                &settings,
                            );
                            status.increase("score", CONTROL_HIT_SCORE);
                            status.update_chain(status::StatusChain::Control, u_pos);
                            p.state = hyper::setup_particle_starting(&mut commands, &p);
                        }
                        _ => (),
                    },
                    ParticleType::Hyper => {
                        if let HitAction::UouHit = action {
                            insert_anime_effect(
                                &mut commands,
                                AnimeEffectParam {
                                    kind: AnimeEffectKind::Explosion,
                                    shape: AnimeEffectShape::Hexagon,
                                    start_pos: p.pos(),
                                    end_pos: p.pos(),
                                },
                            );
                            if let Some(pos) = status.prev_chain_pos(status::StatusChain::Hyper) {
                                insert_anime_effect(
                                    &mut commands,
                                    AnimeEffectParam {
                                        kind: AnimeEffectKind::Bullet,
                                        shape: AnimeEffectShape::Hexagon,
                                        start_pos: pos,
                                        end_pos: p.pos(),
                                    },
                                );
                            };
                            let new_c_pos = field::gen_random_pos_in_field(p.radius);
                            control::build_particle_sprite(
                                &mut commands,
                                reactor::RunningParticle,
                                Some(new_c_pos),
                                Some(new_c_pos - u_pos),
                                Some(p.level()),
                            );
                            status.increase("total_control_count", 1);
                            status.increase("total_hyper_count", 1);
                            p.update_level(1);
                            status.compare_and_update_max_field("hyper_level", p.level() as u32);
                            let h_pos = field::gen_random_pos_in_field(p.radius);
                            p.jump(h_pos);
                            t.translation.x = h_pos.x;
                            t.translation.y = h_pos.y;
                            p.assign_random_v(Some(h_pos - u_pos));
                            app::audio::play_se(
                                app::audio::AudioSe::PowerUp,
                                &mut commands,
                                &audio_se_asset,
                                &settings,
                            );
                            status.increase("score", HYPER_HIT_BASE_SCORE * p.level() as u32);
                            status.update_chain(status::StatusChain::Hyper, u_pos);
                            p.state = hyper::setup_particle_starting(&mut commands, &p);
                        }
                    }
                    ParticleType::Uou => {
                        if let HitAction::Kill = action {
                            #[cfg(not(target_arch = "wasm32"))]
                            app::screenshot::shot_current(
                                &main_window,
                                &mut screenshot_manager,
                                "score",
                            );
                            app::audio::play_se(
                                app::audio::AudioSe::Boom,
                                &mut commands,
                                &audio_se_asset,
                                &settings,
                            );
                            reactor_state.set(reactor::ReactorState::Submit);
                        }
                    }
                    _ => (),
                }
            }
        }
        for tween_event in tween_completed_events.read() {
            if tween_event.user_data == STARTING_DONE_EVENT {
                for (e, mut p, _) in particle_query.iter_mut() {
                    if e == tween_event.entity {
                        p.state_starting_done(&mut commands);
                    }
                }
            }
            if tween_event.user_data == ENDING_DONE_EVENT
                || tween_event.user_data == ANIME_EFFECT_DONE_EVENT
            {
                entities_to_despawn.insert(tween_event.entity);
            }
        }
        for entity in entities_to_despawn.iter() {
            if let Some(entity_commands) = commands.get_entity(*entity) {
                entity_commands.despawn_recursive()
            }
        }
    }
}

fn control_u_by_mouse(
    mut panel_query: Query<&Interaction, (With<Interaction>, With<GameControlPanel>)>,
    mut u_particle_query: Query<(&mut Particle, &mut Transform), With<reactor::ControlParticle>>,
    mut mouse_motion_events: EventReader<input::mouse::MouseMotion>,
    settings: Res<Persistent<app::settings::Settings>>,
    status: Res<reactor::status::ReactorStatus>,
) {
    for interaction in &mut panel_query {
        if *interaction == Interaction::Pressed {
            let events = mouse_motion_events.read().collect::<Vec<_>>();
            for event in events.iter().rev().take(3) {
                move_u(event.delta, &mut u_particle_query, &status, &settings);
            }
        }
    }
}

const KEYBOARD_DELTA_BIAS: f32 = 1.5;

fn control_u_by_keyboard(
    keyboard_input: Res<Input<KeyCode>>,
    mut u_particle_query: Query<(&mut Particle, &mut Transform), With<reactor::ControlParticle>>,
    settings: Res<Persistent<app::settings::Settings>>,
    status: Res<reactor::status::ReactorStatus>,
) {
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
    move_u(delta, &mut u_particle_query, &status, &settings);
}

const GAEMPAD_DELTA_BIAS: f32 = 2.0;
const GAEMPAD_MIN_THRESHOLD: f32 = 0.25;

fn control_u_by_gamepad(
    mut events: EventReader<input::gamepad::GamepadEvent>,
    mut u_particle_query: Query<(&mut Particle, &mut Transform), With<reactor::ControlParticle>>,
    mut last_delta: Local<Vec2>,
    settings: Res<Persistent<app::settings::Settings>>,
    status: Res<reactor::status::ReactorStatus>,
) {
    for event in events.read() {
        dbg!(event);
        if let input::gamepad::GamepadEvent::Axis(axis_event) = event {
            let mut delta: Vec2 = Vec2::default();
            if axis_event.axis_type == input::gamepad::GamepadAxisType::LeftStickX
                || axis_event.axis_type == input::gamepad::GamepadAxisType::RightStickX
            {
                delta.x = GAEMPAD_DELTA_BIAS * axis_event.value;
                if axis_event.value.abs() > GAEMPAD_MIN_THRESHOLD {
                    last_delta.x = delta.x;
                } else {
                    last_delta.x = 0.0;
                }
            }
            if axis_event.axis_type == input::gamepad::GamepadAxisType::LeftStickY
                || axis_event.axis_type == input::gamepad::GamepadAxisType::RightStickY
            {
                delta.y = GAEMPAD_DELTA_BIAS * axis_event.value * -1.0;
                if axis_event.value.abs() > GAEMPAD_MIN_THRESHOLD {
                    last_delta.y = delta.y;
                } else {
                    last_delta.y = 0.0;
                }
            }
        }
    }
    move_u(*last_delta, &mut u_particle_query, &status, &settings);
}

fn move_u(
    delta: Vec2,
    u_particle_query: &mut Query<(&mut Particle, &mut Transform), With<reactor::ControlParticle>>,
    status: &Res<reactor::status::ReactorStatus>,
    settings: &Res<Persistent<app::settings::Settings>>,
) {
    let (mut u_particle, mut u_transform) = u_particle_query.get_single_mut().unwrap();
    let new_pos = calculate_u_new_pos(
        u_particle.pos(),
        delta,
        if status.in_modified_sensitivity {
            settings.get_value("sensitivity_modified")
        } else {
            settings.get_value("sensitivity")
        },
    );
    u_particle.jump(new_pos);
    u_transform.translation.x = new_pos.x;
    u_transform.translation.y = new_pos.y;
}

fn calculate_u_new_pos(current: Vec2, delta: Vec2, sensitivity: u8) -> Vec2 {
    let delta_ratio = 0.5 + sensitivity as f32 / 100.0 * 5.0;
    let field_rect = field::get_field_rect(uou::RADIUS + 3.0);
    let new_x = (current.x + delta.x * delta_ratio).clamp(field_rect.min.x, field_rect.max.x);
    let new_y = (current.y - delta.y * delta_ratio).clamp(field_rect.min.y, field_rect.max.y);
    Vec2::new(new_x, new_y)
}

fn detect_sensitivity_modification(
    keyboard_input: Res<Input<KeyCode>>,
    mut button_changed_events: EventReader<input::gamepad::GamepadButtonChangedEvent>,
    mut status: ResMut<reactor::status::ReactorStatus>,
) {
    for btn_event in button_changed_events.read() {
        if btn_event.button_type == input::gamepad::GamepadButtonType::RightTrigger
            || btn_event.button_type == input::gamepad::GamepadButtonType::LeftTrigger
        {
            status.in_modified_sensitivity = btn_event.value == 1.0;
        }
    }
    if keyboard_input.just_pressed(KeyCode::ShiftLeft)
        || keyboard_input.just_pressed(KeyCode::ShiftRight)
    {
        status.in_modified_sensitivity = true;
    }
    if keyboard_input.just_released(KeyCode::ShiftLeft)
        || keyboard_input.just_released(KeyCode::ShiftRight)
    {
        status.in_modified_sensitivity = false;
    }
}
