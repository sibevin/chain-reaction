use crate::{app, reactor};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

const FIELD_TEXT_SIZE: f32 = reactor::FIELD_NAV_H * 0.5;
const FIELD_PADDING: f32 = (reactor::FIELD_NAV_H - FIELD_TEXT_SIZE) / 2.0;

const FIELD_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
const FIELD_TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

#[derive(Component)]
pub struct FieldTimer;

#[derive(Component)]
pub struct FieldAlphaCount;

#[derive(Component)]
pub struct FieldScore;

#[derive(Component)]
pub struct FieldIconChain;

#[derive(Component)]
pub struct FieldChain;

pub fn get_field_rect(padding: f32) -> Rect {
    Rect::new(
        -reactor::FIELD_W / 2.0 + padding,
        (-reactor::FIELD_H + reactor::FIELD_NAV_H) / 2.0 + padding,
        reactor::FIELD_W / 2.0 - padding,
        (reactor::FIELD_H + reactor::FIELD_NAV_H) / 2.0 - padding,
    )
}

pub fn gen_random_pos_in_field(padding: f32) -> Vec2 {
    let mut rng = thread_rng();
    let rect = get_field_rect(padding);
    Vec2::new(
        rng.gen_range(rect.min.x..rect.max.x),
        rng.gen_range(rect.min.y..rect.max.y),
    )
}

pub fn format_field_text(field: &str, value: u32) -> String {
    if field == "time" {
        format!("{:0>5}.{:0>2}", value / 100, value % 100)
    } else if field == "alpha_count" {
        format!("{:0>4}", value)
    } else if field == "score" {
        let value_str = format!("{:0>6}", value);
        let (first, second) = value_str.split_at(3);
        format!("{},{}", first, second)
    } else if field == "chain" {
        format!("{:0>4}", value)
    } else {
        format!("{}", value)
    }
}

pub fn build_reactor_field(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn((NodeBundle {
                    style: Style {
                        width: Val::Px(app::WINDOW_W),
                        height: Val::Px(app::WINDOW_H),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        border: UiRect::all(app::ui::px_p(0.5)),
                        ..default()
                    },
                    border_color: FIELD_COLOR.into(),
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn((NodeBundle {
                        style: Style {
                            flex_grow: 1.0,
                            ..default()
                        },
                        ..default()
                    },));
                    parent
                        .spawn((NodeBundle {
                            style: Style {
                                height: Val::Px(reactor::FIELD_NAV_H),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceBetween,
                                border: UiRect::top(app::ui::px_p(0.5)),
                                padding: UiRect::horizontal(Val::Px(FIELD_PADDING * 1.4)),
                                column_gap: Val::Px(FIELD_PADDING * 1.4),
                                ..default()
                            },
                            border_color: FIELD_COLOR.into(),
                            ..default()
                        },))
                        .with_children(|parent| {
                            parent.spawn((NodeBundle {
                                style: Style {
                                    width: Val::Px(FIELD_TEXT_SIZE * 0.6),
                                    height: Val::Px(FIELD_TEXT_SIZE * 0.6),
                                    border: UiRect::all(app::ui::px_p(0.5)),
                                    ..default()
                                },
                                border_color: FIELD_COLOR.into(),
                                ..default()
                            },));
                            parent
                                .spawn((NodeBundle {
                                    style: Style {
                                        flex_grow: 1.0,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::SpaceBetween,
                                        ..default()
                                    },
                                    ..default()
                                },))
                                .with_children(|parent| {
                                    parent
                                        .spawn((NodeBundle {
                                            style: Style {
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::Start,
                                                ..default()
                                            },
                                            ..default()
                                        },))
                                        .with_children(|parent| {
                                            let icon =
                                                asset_server.load("images/icons/trophy-fill.png");
                                            parent.spawn(ImageBundle {
                                                style: Style {
                                                    width: Val::Px(FIELD_TEXT_SIZE),
                                                    height: Val::Px(FIELD_TEXT_SIZE),
                                                    margin: UiRect::right(Val::Px(
                                                        FIELD_PADDING * 0.5,
                                                    )),
                                                    ..default()
                                                },
                                                image: UiImage::new(icon),
                                                ..default()
                                            });
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    format_field_text("score", 0),
                                                    TextStyle {
                                                        font: asset_server
                                                            .load(app::ui::FONT_DIGIT),
                                                        font_size: FIELD_TEXT_SIZE,
                                                        color: FIELD_TEXT_COLOR,
                                                        ..default()
                                                    },
                                                ),
                                                FieldScore,
                                            ));
                                        });
                                    parent
                                        .spawn((NodeBundle {
                                            style: Style {
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::Center,
                                                ..default()
                                            },
                                            ..default()
                                        },))
                                        .with_children(|parent| {
                                            let icon =
                                                asset_server.load("images/icons/timer-fill.png");
                                            parent.spawn(ImageBundle {
                                                style: Style {
                                                    width: Val::Px(FIELD_TEXT_SIZE),
                                                    height: Val::Px(FIELD_TEXT_SIZE),
                                                    margin: UiRect::right(Val::Px(
                                                        FIELD_PADDING * 0.5,
                                                    )),
                                                    ..default()
                                                },
                                                image: UiImage::new(icon),
                                                ..default()
                                            });
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    format_field_text("time", 0),
                                                    TextStyle {
                                                        font: asset_server
                                                            .load(app::ui::FONT_DIGIT),
                                                        font_size: FIELD_TEXT_SIZE,
                                                        color: FIELD_TEXT_COLOR,
                                                        ..default()
                                                    },
                                                ),
                                                FieldTimer,
                                            ));
                                        });
                                    parent
                                        .spawn((NodeBundle {
                                            style: Style {
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::Center,
                                                ..default()
                                            },
                                            ..default()
                                        },))
                                        .with_children(|parent| {
                                            let icon = asset_server
                                                .load("images/icons/circles-three-fill.png");
                                            parent.spawn(ImageBundle {
                                                style: Style {
                                                    width: Val::Px(FIELD_TEXT_SIZE),
                                                    height: Val::Px(FIELD_TEXT_SIZE),
                                                    margin: UiRect::right(Val::Px(
                                                        FIELD_PADDING * 0.5,
                                                    )),
                                                    ..default()
                                                },
                                                image: UiImage::new(icon),
                                                ..default()
                                            });
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    format_field_text("alpha_count", 0),
                                                    TextStyle {
                                                        font: asset_server
                                                            .load(app::ui::FONT_DIGIT),
                                                        font_size: FIELD_TEXT_SIZE,
                                                        color: FIELD_TEXT_COLOR,
                                                        ..default()
                                                    },
                                                ),
                                                FieldAlphaCount,
                                            ));
                                        });
                                    parent
                                        .spawn((NodeBundle {
                                            style: Style {
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::Center,
                                                ..default()
                                            },
                                            ..default()
                                        },))
                                        .with_children(|parent| {
                                            let icon =
                                                asset_server.load("images/icons/line-segments.png");
                                            parent.spawn(ImageBundle {
                                                style: Style {
                                                    width: Val::Px(FIELD_TEXT_SIZE),
                                                    height: Val::Px(FIELD_TEXT_SIZE),
                                                    margin: UiRect::right(Val::Px(
                                                        FIELD_PADDING * 0.5,
                                                    )),
                                                    ..default()
                                                },
                                                image: UiImage::new(icon),
                                                ..default()
                                            });
                                            let icon = asset_server.load("images/icons/circle.png");
                                            parent.spawn((
                                                ImageBundle {
                                                    style: Style {
                                                        width: Val::Px(FIELD_TEXT_SIZE),
                                                        height: Val::Px(FIELD_TEXT_SIZE),
                                                        margin: UiRect::right(Val::Px(
                                                            FIELD_PADDING * 0.5,
                                                        )),
                                                        ..default()
                                                    },
                                                    image: UiImage::new(icon),
                                                    ..default()
                                                },
                                                FieldIconChain,
                                            ));
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    format_field_text("chain", 0),
                                                    TextStyle {
                                                        font: asset_server
                                                            .load(app::ui::FONT_DIGIT),
                                                        font_size: FIELD_TEXT_SIZE,
                                                        color: FIELD_TEXT_COLOR,
                                                        ..default()
                                                    },
                                                ),
                                                FieldChain,
                                            ));
                                        });
                                });
                        });
                });
        })
        .id()
}

const SCORE_PER_SECOND: u32 = 10;

pub fn update_reactor_field(
    time: Res<Time>,
    mut reactor_timer_query: Query<&mut reactor::ReactorTimer>,
    mut score_timer_query: Query<&mut reactor::ScoreTimer>,
    mut field_timer_query: Query<
        &mut Text,
        (
            With<FieldTimer>,
            Without<FieldScore>,
            Without<FieldAlphaCount>,
            Without<FieldChain>,
        ),
    >,
    mut field_score_query: Query<
        &mut Text,
        (
            With<FieldScore>,
            Without<FieldTimer>,
            Without<FieldAlphaCount>,
            Without<FieldChain>,
        ),
    >,
    mut field_ac_query: Query<
        &mut Text,
        (
            With<FieldAlphaCount>,
            Without<FieldTimer>,
            Without<FieldScore>,
            Without<FieldChain>,
        ),
    >,
    mut field_chain_query: Query<
        &mut Text,
        (
            With<FieldChain>,
            Without<FieldTimer>,
            Without<FieldScore>,
            Without<FieldAlphaCount>,
        ),
    >,
    mut field_icon_chain_query: Query<&mut UiImage, (With<FieldIconChain>,)>,
    mut status: ResMut<reactor::status::ReactorStatus>,
    particle_query: Query<&reactor::particle::Particle, With<reactor::particle::Particle>>,
    asset_server: Res<AssetServer>,
) {
    let mut reactor_timer = reactor_timer_query.single_mut();
    if reactor_timer.tick(time.delta()).just_finished() {
        let time = status.increase("time", 1);
        let mut text = field_timer_query.single_mut();
        text.sections[0].value = format_field_text("time", time);
        let mut total_alpha_count = 0;
        for particle in particle_query.iter() {
            if particle.particle_type() == reactor::particle::ParticleType::Alpha {
                total_alpha_count += 1;
            }
        }
        status.update("alpha_count", total_alpha_count);
        status.compare_and_update_max_field("alpha_count", total_alpha_count);
        let mut text = field_ac_query.single_mut();
        text.sections[0].value = format_field_text("alpha_count", total_alpha_count);
        let mut text = field_chain_query.single_mut();
        text.sections[0].value = format_field_text("chain", status.fetch("chain_length"));
        text.sections[0].style.color = match status.current_chain() {
            reactor::status::StatusChain::Control => reactor::particle::control::COLOR,
            reactor::status::StatusChain::None => FIELD_TEXT_COLOR,
            reactor::status::StatusChain::Hyper => reactor::particle::hyper::COLOR,
        };
        let mut image = field_icon_chain_query.single_mut();
        image.texture = match status.current_chain() {
            reactor::status::StatusChain::Control => asset_server.load("images/icons/square.png"),
            reactor::status::StatusChain::None => asset_server.load("images/icons/circle.png"),
            reactor::status::StatusChain::Hyper => asset_server.load("images/icons/hexagon.png"),
        }
    }
    let mut score_timer = score_timer_query.single_mut();
    if score_timer.tick(time.delta()).just_finished() {
        let alpha_count = status.fetch("alpha_count");
        status.increase("score", alpha_count);
        let score = status.increase("score", SCORE_PER_SECOND);
        let mut text = field_score_query.single_mut();
        text.sections[0].value = format_field_text("score", score);
    }
}

pub fn reset_reactor_field(
    mut field_timer_query: Query<
        &mut Text,
        (
            With<FieldTimer>,
            Without<FieldScore>,
            Without<FieldAlphaCount>,
        ),
    >,
    mut field_score_query: Query<
        &mut Text,
        (
            With<FieldScore>,
            Without<FieldTimer>,
            Without<FieldAlphaCount>,
        ),
    >,
    mut field_ac_query: Query<
        &mut Text,
        (
            With<FieldAlphaCount>,
            Without<FieldTimer>,
            Without<FieldScore>,
        ),
    >,
    mut status: ResMut<reactor::status::ReactorStatus>,
) {
    let mut text = field_timer_query.single_mut();
    text.sections[0].value = format_field_text("time", 0);
    let mut text = field_score_query.single_mut();
    text.sections[0].value = format_field_text("score", 0);
    let mut text = field_ac_query.single_mut();
    text.sections[0].value = format_field_text("alpha_count", 0);
    status.reset();
}
