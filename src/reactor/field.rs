use crate::{app, page, reactor};
use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy::{render::view::window::screenshot::ScreenshotManager, window::PrimaryWindow};
use bevy_persistent::prelude::*;
use rand::{thread_rng, Rng};

const FIELD_TEXT_SIZE: f32 = reactor::FIELD_NAV_H * 0.5;
const FIELD_PADDING: f32 = (reactor::FIELD_NAV_H - FIELD_TEXT_SIZE) / 2.0;

const FIELD_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
const FIELD_TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

pub const REACTOR_FIELDS: [&str; 4] = ["score", "time", "alpha_count", "chain"];

#[derive(Component)]
pub struct ReactorField(String);

#[derive(Component)]
pub struct ReactorChainIcon;

#[derive(Component)]
pub struct TargetRankField(String);

#[derive(Component)]
pub struct TargetValueField(String);

#[derive(Component)]
pub struct TargetBar(String);

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
        format!("{:0>4}.{:0>2}", value / 100, value % 100)
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

const TARGET_TEXT_SIZE: f32 = app::ui::FONT_SIZE * 0.8;
const TARGET_COLOR_ALPHA: f32 = 0.2;
const TARGET_COLOR: Color = Color::rgba(0.5, 0.5, 0.5, TARGET_COLOR_ALPHA);
const TARGET_BG_COLOR: Color = Color::rgba(0.5, 0.5, 0.5, TARGET_COLOR_ALPHA * 0.5);

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
                    parent
                        .spawn((NodeBundle {
                            style: Style {
                                flex_grow: 1.0,
                                ..default()
                            },
                            ..default()
                        },))
                        .with_children(|parent| build_target_fields(parent, &asset_server));

                    build_reactor_fields(parent, &asset_server);
                });
        })
        .id()
}

fn build_target_fields(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: app::ui::px_p(page::PAGE_PADDING),
                right: app::ui::px_p(page::PAGE_PADDING),
                bottom: Val::Px(0.0),
                padding: UiRect::bottom(app::ui::px_p(4.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                column_gap: app::ui::px_p(8.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            for field in app::leaderboard::LEADERBOARD_LISTS {
                let flex_grow = match field {
                    "score" | "time" | "max_alpha_count" => 1.2,
                    "max_control_chain" | "max_hyper_chain" => 0.3,
                    _ => panic!("Invalid field"),
                };
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_grow,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Stretch,
                            justify_content: JustifyContent::End,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::SpaceBetween,
                                    column_gap: app::ui::px_p(3.0),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                let (target_rank, target_value) = (0, 0);
                                let target_value_text = match field {
                                    "score" => format_field_text("score", target_value),
                                    "time" => format_field_text("time", target_value),
                                    "max_alpha_count" => {
                                        format_field_text("alpha_count", target_value)
                                    }
                                    "max_control_chain" => format_field_text("chain", target_value),
                                    "max_hyper_chain" => format_field_text("chain", target_value),
                                    _ => panic!("Invalid field"),
                                };
                                let target_rank_text = match target_rank {
                                    0 => String::from("TOP"),
                                    1 => String::from("1st"),
                                    2 => String::from("2nd"),
                                    3 => String::from("3rd"),
                                    _ => format!("{}th", target_rank),
                                };
                                let number_color = match field {
                                    "score" | "time" | "max_alpha_count" => TARGET_COLOR,
                                    "max_control_chain" => *reactor::particle::control::COLOR
                                        .clone()
                                        .set_a(TARGET_COLOR_ALPHA),
                                    "max_hyper_chain" => *reactor::particle::hyper::COLOR
                                        .clone()
                                        .set_a(TARGET_COLOR_ALPHA),
                                    _ => panic!("Invalid field"),
                                };
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            padding: UiRect::all(app::ui::px_p(3.0)),
                                            ..default()
                                        },
                                        background_color: TARGET_COLOR.into(),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn((
                                            TextBundle::from_section(
                                                format!("{}", target_rank_text),
                                                TextStyle {
                                                    font: asset_server.load(app::ui::FONT_DIGIT),
                                                    font_size: TARGET_TEXT_SIZE * 0.6,
                                                    color: app::ui::BG_COLOR,
                                                    ..default()
                                                },
                                            ),
                                            TargetRankField(String::from(field)),
                                        ));
                                    });
                                parent.spawn((
                                    TextBundle::from_section(
                                        target_value_text,
                                        TextStyle {
                                            font: asset_server.load(app::ui::FONT_DIGIT),
                                            font_size: TARGET_TEXT_SIZE,
                                            color: number_color,
                                            ..default()
                                        },
                                    ),
                                    TargetValueField(String::from(field)),
                                ));
                            });
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Start,
                                    ..default()
                                },
                                background_color: TARGET_BG_COLOR.into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Percent(0.0),
                                            height: app::ui::px_p(2.0),
                                            ..default()
                                        },
                                        background_color: TARGET_COLOR.into(),
                                        ..default()
                                    },
                                    TargetBar(String::from(field)),
                                ));
                            });
                    });
            }
        });
}

fn build_reactor_fields(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
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
                    for field in REACTOR_FIELDS {
                        let icon_path = match field {
                            "score" => "images/icons/trophy-fill.png",
                            "time" => "images/icons/timer-fill.png",
                            "alpha_count" => "images/icons/circles-three-fill.png",
                            "chain" => "images/icons/line-segments.png",
                            _ => panic!("Invalid field"),
                        };
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
                                let icon = asset_server.load(icon_path);
                                parent.spawn(ImageBundle {
                                    style: Style {
                                        width: Val::Px(FIELD_TEXT_SIZE),
                                        height: Val::Px(FIELD_TEXT_SIZE),
                                        margin: UiRect::right(Val::Px(FIELD_PADDING * 0.5)),
                                        ..default()
                                    },
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                if field == "chain" {
                                    let icon = asset_server.load("images/icons/circle.png");
                                    parent.spawn((
                                        ImageBundle {
                                            style: Style {
                                                width: Val::Px(FIELD_TEXT_SIZE),
                                                height: Val::Px(FIELD_TEXT_SIZE),
                                                margin: UiRect::right(Val::Px(FIELD_PADDING * 0.5)),
                                                ..default()
                                            },
                                            image: UiImage::new(icon),
                                            ..default()
                                        },
                                        ReactorChainIcon,
                                    ));
                                }
                                parent.spawn((
                                    TextBundle::from_section(
                                        format_field_text(field, 0),
                                        TextStyle {
                                            font: asset_server.load(app::ui::FONT_DIGIT),
                                            font_size: FIELD_TEXT_SIZE,
                                            color: FIELD_TEXT_COLOR,
                                            ..default()
                                        },
                                    ),
                                    ReactorField(String::from(field)),
                                ));
                            });
                    }
                });
        });
}

const SCORE_PER_SECOND: u32 = 10;

pub fn update_reactor_fields(
    time: Res<Time>,
    mut reactor_fields_query: Query<(&mut Text, &ReactorField), With<ReactorField>>,
    mut reactor_chain_icon_query: Query<&mut UiImage, With<ReactorChainIcon>>,
    mut reactor_timer_query: Query<&mut reactor::ReactorTimer>,
    mut score_timer_query: Query<&mut reactor::ScoreTimer>,
    mut status: ResMut<reactor::status::ReactorStatus>,
    particle_query: Query<&reactor::particle::Particle, With<reactor::particle::Particle>>,
    asset_server: Res<AssetServer>,
    #[cfg(not(target_arch = "wasm32"))] main_window: Query<Entity, With<PrimaryWindow>>,
    #[cfg(not(target_arch = "wasm32"))] mut screenshot_manager: ResMut<ScreenshotManager>,
    #[cfg(not(target_arch = "wasm32"))] reactor_status: Res<State<reactor::ReactorState>>,
) {
    let mut reactor_timer = reactor_timer_query.single_mut();
    if reactor_timer.tick(time.delta()).just_finished() {
        let mut control_count = 0;
        let mut full_level_control_count = 0;
        for particle in particle_query.iter() {
            if particle.particle_type() == reactor::particle::ParticleType::Control {
                control_count += 1;
                if particle.level() == reactor::particle::control::MAX_LEVEL {
                    full_level_control_count += 1;
                }
            }
        }
        status.compare_and_update_max_field("control_count", control_count);
        status.compare_and_update_max_field("full_level_control_count", full_level_control_count);
        for (mut text, field) in reactor_fields_query.iter_mut() {
            match field.0.as_ref() {
                "score" => {
                    text.sections[0].value = format_field_text("score", status.fetch("score"));
                }
                "time" => {
                    let time = status.increase("time", 1);
                    text.sections[0].value = format_field_text("time", time);
                }
                "alpha_count" => {
                    let mut total_alpha_count = 0;
                    for particle in particle_query.iter() {
                        if particle.particle_type() == reactor::particle::ParticleType::Alpha {
                            total_alpha_count += 1;
                        }
                    }
                    status.update("alpha_count", total_alpha_count);
                    if status.compare_and_update_max_field("alpha_count", total_alpha_count) {
                        #[cfg(not(target_arch = "wasm32"))]
                        if reactor_status.eq(&reactor::ReactorState::Running) {
                            app::screenshot::shot_current(
                                &main_window,
                                &mut screenshot_manager,
                                "max_alpha",
                            );
                        }
                    }
                    text.sections[0].value = format_field_text("alpha_count", total_alpha_count);
                }
                "chain" => {
                    text.sections[0].value =
                        format_field_text("chain", status.fetch("chain_length"));
                    text.sections[0].style.color = match status.current_chain() {
                        reactor::status::StatusChain::Control => reactor::particle::control::COLOR,
                        reactor::status::StatusChain::None => FIELD_TEXT_COLOR,
                        reactor::status::StatusChain::Hyper => reactor::particle::hyper::COLOR,
                    };
                }
                _ => (),
            }
        }
        let mut image = reactor_chain_icon_query.single_mut();
        image.texture = match status.current_chain() {
            reactor::status::StatusChain::Control => asset_server.load("images/icons/square.png"),
            reactor::status::StatusChain::None => asset_server.load("images/icons/circle.png"),
            reactor::status::StatusChain::Hyper => asset_server.load("images/icons/hexagon.png"),
        };
    }

    let mut score_timer = score_timer_query.single_mut();
    if score_timer.tick(time.delta()).just_finished() {
        for (mut text, field) in reactor_fields_query.iter_mut() {
            match field.0.as_ref() {
                "score" => {
                    let alpha_count = status.fetch("alpha_count");
                    status.increase("score", alpha_count);
                    let score = status.increase("score", SCORE_PER_SECOND);
                    text.sections[0].value = format_field_text("score", score);
                }
                _ => (),
            }
        }
    }
}

pub fn reset_reactor_fields(
    mut reactor_fields_query: Query<(&mut Text, &ReactorField), With<ReactorField>>,
    mut reactor_chain_icon_query: Query<&mut UiImage, With<ReactorChainIcon>>,
    mut status: ResMut<reactor::status::ReactorStatus>,
    asset_server: Res<AssetServer>,
) {
    for (mut text, field) in reactor_fields_query.iter_mut() {
        text.sections[0].value = format_field_text(field.0.as_ref(), 0);
    }
    let mut image = reactor_chain_icon_query.single_mut();
    image.texture = asset_server.load("images/icons/circle.png");
    status.reset();
}

pub fn update_target_fields(
    mut target_rank_fields_query: Query<
        (&mut Text, &TargetRankField),
        (With<TargetRankField>, Without<TargetValueField>),
    >,
    mut target_value_fields_query: Query<(&mut Text, &TargetValueField), With<TargetValueField>>,
    mut target_bars_query: Query<(&mut Style, &TargetBar), With<TargetBar>>,
    status: Res<reactor::status::ReactorStatus>,
    leaderboard: Res<Persistent<app::leaderboard::Leaderboard>>,
) {
    for (mut text, field) in target_rank_fields_query.iter_mut() {
        let number = status.fetch(field.0.as_ref());
        let (target_rank, _, _) = leaderboard.target(field.0.as_ref(), number);
        let target_rank_text = match target_rank {
            0 => String::from("TOP"),
            1 => String::from("1st"),
            2 => String::from("2nd"),
            3 => String::from("3rd"),
            _ => format!("{}th", target_rank),
        };
        text.sections[0].value = target_rank_text;
    }
    for (mut text, field) in target_value_fields_query.iter_mut() {
        let number = status.fetch(field.0.as_ref());
        let (target_rank, target_value, _) = leaderboard.target(field.0.as_ref(), number);
        let shown_value = if target_rank == 0 {
            number
        } else {
            target_value
        };
        let target_value_text = match field.0.as_ref() {
            "score" => format_field_text("score", shown_value),
            "time" => format_field_text("time", shown_value),
            "max_alpha_count" => format_field_text("alpha_count", shown_value),
            "max_control_chain" => format_field_text("chain", shown_value),
            "max_hyper_chain" => format_field_text("chain", shown_value),
            _ => panic!("Invalid field"),
        };
        text.sections[0].value = target_value_text;
    }
    for (mut style, bar) in target_bars_query.iter_mut() {
        let number = status.fetch(bar.0.as_ref());
        let (_, target_value, prev_value) = leaderboard.target(bar.0.as_ref(), number);
        let bar_precent = if target_value == prev_value {
            100.0
        } else if number - prev_value <= 0 {
            0.0
        } else {
            (number - prev_value) as f32 / (target_value - prev_value) as f32 * 100.0
        };
        style.width = Val::Percent(bar_precent);
    }
}

pub fn reset_target_fields(
    mut target_rank_fields_query: Query<
        (&mut Text, &TargetRankField),
        (With<TargetRankField>, Without<TargetValueField>),
    >,
    mut target_value_fields_query: Query<(&mut Text, &TargetValueField), With<TargetValueField>>,
    mut target_bars_query: Query<&mut Style, With<TargetBar>>,
    leaderboard: Res<Persistent<app::leaderboard::Leaderboard>>,
) {
    for (mut text, field) in target_rank_fields_query.iter_mut() {
        let (target_rank, _, _) = leaderboard.target(field.0.as_ref(), 0);
        let target_rank_text = match target_rank {
            0 => String::from("TOP"),
            1 => String::from("1st"),
            2 => String::from("2nd"),
            3 => String::from("3rd"),
            _ => format!("{}th", target_rank),
        };
        text.sections[0].value = target_rank_text;
    }
    for (mut text, field) in target_value_fields_query.iter_mut() {
        let (_, target_value, _) = leaderboard.target(field.0.as_ref(), 0);
        let target_value_text = match field.0.as_ref() {
            "score" => format_field_text("score", target_value),
            "time" => format_field_text("time", target_value),
            "max_alpha_count" => format_field_text("alpha_count", target_value),
            "max_control_chain" => format_field_text("chain", target_value),
            "max_hyper_chain" => format_field_text("chain", target_value),
            _ => panic!("Invalid field"),
        };
        text.sections[0].value = target_value_text;
    }
    for mut style in target_bars_query.iter_mut() {
        style.width = Val::Percent(0.0);
    }
}
