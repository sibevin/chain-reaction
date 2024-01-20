use crate::{app, reactor};
use bevy::prelude::*;
use bevy_persistent::prelude::*;
use bevy_tweening::*;
use std::time::Duration;

const ACH_ICON_SIZE: f32 = app::ui::FONT_SIZE * 2.5;
const ACH_STATUS_SIZE: f32 = app::ui::FONT_SIZE * 1.0;
const ACH_PANEL_W: f32 = app::ui::FONT_SIZE * 12.0;
const ACH_DESC_FS: f32 = app::ui::FONT_SIZE * 0.8;
const ACH_NAME_FS: f32 = app::ui::FONT_SIZE * 1.2;
const ACH_COLOR_ALPHA: f32 = 1.0;

const ACH_DONE_ANIME_END_EVENT: u64 = 3;

pub fn reset_ach_fields(
    mut commands: Commands,
    ach_running_panel: Query<Entity, With<reactor::field::ReactorAchRunning>>,
    ach_done_panel: Query<Entity, With<reactor::field::ReactorAchDone>>,
    mut ach_info: ResMut<app::achievement::AchievementInfo>,
    ach_store: ResMut<Persistent<app::achievement::AchievementStore>>,
    asset_server: Res<AssetServer>,
    status: ResMut<reactor::status::ReactorStatus>,
) {
    ach_info.reset(&ach_store);
    let running_panel_entity = ach_running_panel.single();
    if let Some(mut entity_commands) = commands.get_entity(running_panel_entity) {
        entity_commands.despawn_descendants();
        entity_commands.with_children(|parent| {
            for code in ach_info.running_codes() {
                build_ach_running_ui(parent, &asset_server, &status, &ach_store, &code);
            }
        });
    }
    let done_panel_entity = ach_done_panel.single();
    if let Some(mut entity_commands) = commands.get_entity(done_panel_entity) {
        entity_commands.despawn_descendants();
        entity_commands.with_children(|parent| {
            let done_code = ach_info.next_done();
            if let Some(code) = done_code {
                build_ach_done_ui(parent, &asset_server, code.as_str());
            }
        });
    }
}

#[allow(clippy::too_many_arguments)]
pub fn update_ach_fields(
    mut commands: Commands,
    ach_running_panel: Query<Entity, With<reactor::field::ReactorAchRunning>>,
    ach_done_panel: Query<Entity, With<reactor::field::ReactorAchDone>>,
    mut ach_info: ResMut<app::achievement::AchievementInfo>,
    asset_server: Res<AssetServer>,
    mut tween_completed_events: EventReader<TweenCompleted>,
    mut status: ResMut<reactor::status::ReactorStatus>,
    mut ach_store: ResMut<Persistent<app::achievement::AchievementStore>>,
    mut ap_bar_texts: Query<(&mut Text, &AchProgressText), With<AchProgressText>>,
    mut ap_bar_values: Query<(&mut Style, &AchProgressBarValue), With<AchProgressBarValue>>,
    mut ap_dots_panels: Query<(Entity, &AchProgressDotsPanel), With<AchProgressDotsPanel>>,
    mut painter_timer: ResMut<reactor::PainterTimer>,
    time: Res<Time>,
    settings: Res<Persistent<app::settings::Settings>>,
    audio_se_asset: Res<app::audio::AudioSeAsset>,
) {
    if painter_timer.0.tick(time.delta()).just_finished() {
        for (mut text, ap_bar_text) in ap_bar_texts.iter_mut() {
            let code = &ap_bar_text.0;
            let ach_def = app::achievement::fetch_ach_def(code);
            let (current, _, _) = ach_def.check_done(&status);
            text.sections[0].value = ach_def.format_value(current);
        }
        for (mut style, ap_bar_value) in ap_bar_values.iter_mut() {
            let code = &ap_bar_value.0;
            let ach_def = app::achievement::fetch_ach_def(code);
            let (current, total, _) = ach_def.check_done(&status);
            let value_bar_ratio = current as f32 / total as f32 * 100.0;
            style.width = Val::Percent(value_bar_ratio);
        }
        for (entity, ap_bar_dots_panel) in ap_dots_panels.iter_mut() {
            if let Some(mut panel_commands) = commands.get_entity(entity) {
                let code = &ap_bar_dots_panel.0;
                let ach_def = app::achievement::fetch_ach_def(code);
                let (current, total, _) = ach_def.check_done(&status);
                panel_commands.despawn_descendants();
                panel_commands.with_children(|parent| {
                    build_ach_running_progress_dots(parent, (current, total));
                });
            }
        }
    }
    let mut trigger_done_code: Option<String> = None;
    let mut is_running_updated: bool = false;
    for ach_def in app::achievement::ACHIEVEMENTS {
        if !status
            .done_achievements
            .contains(&String::from(ach_def.code()))
        {
            let (_, _, is_done) = ach_def.check_done(&status);
            if is_done {
                ach_store
                    .update(|ach_store| {
                        ach_store.mark_done(ach_def.code());
                    })
                    .expect("failed to mark achievement done");
                status.done_achievements.push(String::from(ach_def.code()));
                if ach_info.is_running(ach_def.code()) {
                    ach_info.update_running_codes(&ach_store);
                    is_running_updated = true;
                }
                trigger_done_code = ach_info.push_to_done(ach_def.code());
                app::audio::play_se(
                    app::audio::AudioSe::Tada,
                    &mut commands,
                    &audio_se_asset,
                    &settings,
                );
            }
        }
    }
    if is_running_updated {
        let running_panel_entity = ach_running_panel.single();
        let mut running_commands = commands.get_entity(running_panel_entity).unwrap();
        running_commands.despawn_descendants();
        running_commands.with_children(|parent| {
            for code in ach_info.running_codes() {
                build_ach_running_ui(parent, &asset_server, &status, &ach_store, &code);
            }
        });
    }
    let done_panel_entity = ach_done_panel.single();
    let mut done_commands = commands.get_entity(done_panel_entity).unwrap();
    if let Some(code) = trigger_done_code {
        done_commands.despawn_descendants();
        done_commands.with_children(|parent| {
            build_ach_done_ui(parent, &asset_server, code.as_str());
        });
    }
    for tween_event in tween_completed_events.read() {
        if tween_event.user_data == ACH_DONE_ANIME_END_EVENT {
            if let Some(code) = ach_info.next_done() {
                done_commands.despawn_descendants();
                done_commands.with_children(|parent| {
                    build_ach_done_ui(parent, &asset_server, code.as_str());
                });
            } else {
                done_commands.despawn_descendants();
            }
        }
    }
}

struct AchRunningAnimeLens {
    start_left: f32,
    end_left: f32,
}

impl lens::Lens<Style> for AchRunningAnimeLens {
    fn lerp(&mut self, style: &mut Style, ratio: f32) {
        style.left = Val::Px(self.start_left + (self.end_left - self.start_left) * ratio);
    }
}

fn build_ach_running_ui(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    status: &ResMut<reactor::status::ReactorStatus>,
    ach_store: &ResMut<Persistent<app::achievement::AchievementStore>>,
    code: &str,
) {
    let ach_def = app::achievement::fetch_ach_def(code);
    let color = app::ui::MUTE_COLOR.with_a(ACH_COLOR_ALPHA);
    let tween = Tween::new(
        EaseFunction::CubicOut,
        Duration::from_millis(500),
        AchRunningAnimeLens {
            start_left: 36.0,
            end_left: 0.0,
        },
    );
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Auto,
                    width: Val::Px(ACH_PANEL_W),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Start,
                    padding: UiRect::all(app::ui::px_p(4.0)),
                    border: UiRect::all(app::ui::px_p(1.0)),
                    column_gap: app::ui::px_p(4.0),
                    ..default()
                },
                background_color: app::ui::BG_COLOR.with_a(ACH_COLOR_ALPHA).into(),
                border_color: color.into(),
                ..default()
            },
            Animator::new(tween),
        ))
        .with_children(|parent| {
            let icon = if ach_store.is_pinned(code) {
                asset_server.load("images/icons/ach-push-pin.png")
            } else {
                asset_server.load("images/icons/ach-crosshair.png")
            };
            parent.spawn(ImageBundle {
                style: Style {
                    align_self: AlignSelf::Start,
                    width: Val::Px(ACH_STATUS_SIZE),
                    height: Val::Px(ACH_STATUS_SIZE),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
            parent
                .spawn((NodeBundle {
                    style: Style {
                        flex_grow: 1.0,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    background_color: app::ui::BG_COLOR.with_a(ACH_COLOR_ALPHA).into(),
                    ..default()
                },))
                .with_children(|parent| {
                    let (value, total, _) = ach_def.check_done(status);
                    parent
                        .spawn((NodeBundle {
                            style: Style {
                                justify_content: JustifyContent::SpaceBetween,
                                ..default()
                            },
                            ..default()
                        },))
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    ach_def.description(),
                                    TextStyle {
                                        font: asset_server.load(app::ui::FONT),
                                        font_size: ACH_DESC_FS,
                                        color: app::ui::SECONDARY_COLOR.with_a(ACH_COLOR_ALPHA),
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::right(app::ui::px_p(0.0)),
                                    ..default()
                                }),
                            );
                            parent.spawn((
                                TextBundle::from_section(
                                    ach_def.format_value(value),
                                    TextStyle {
                                        font: asset_server.load(app::ui::FONT),
                                        font_size: PROGRESS_FS,
                                        color: app::ui::FG_COLOR.with_a(ACH_COLOR_ALPHA),
                                    },
                                )
                                .with_style(Style {
                                    align_self: AlignSelf::End,
                                    ..default()
                                }),
                                AchProgressText(String::from(code)),
                            ));
                        });
                    match ach_def.progress_ui() {
                        app::achievement::AchievementProgressUi::Bar => {
                            build_ach_running_progress_bar(parent, (value, total), code);
                        }
                        app::achievement::AchievementProgressUi::Dots => {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            justify_content: JustifyContent::Start,
                                            align_items: AlignItems::Center,
                                            margin: UiRect::top(app::ui::px_p(4.0)),
                                            column_gap: app::ui::px_p(PROGRESS_BAR_H * 0.5),
                                            ..default()
                                        },
                                        background_color: app::ui::BG_COLOR
                                            .with_a(ACH_COLOR_ALPHA)
                                            .into(),
                                        ..default()
                                    },
                                    AchProgressDotsPanel(String::from(code)),
                                ))
                                .with_children(|parent| {
                                    build_ach_running_progress_dots(parent, (value, total));
                                });
                        }
                    }
                });
        });
}

#[derive(Component)]
pub struct AchProgressText(String);

#[derive(Component)]
pub struct AchProgressBarValue(String);

#[derive(Component)]
pub struct AchProgressDotsPanel(String);

const PROGRESS_BAR_H: f32 = 3.0;
const PROGRESS_FS: f32 = app::ui::FONT_SIZE * 0.8;

fn build_ach_running_progress_bar(parent: &mut ChildBuilder, value_total: (u32, u32), code: &str) {
    let (value, total) = value_total;
    let value_bar_ratio = value as f32 / total as f32 * 100.0;
    parent
        .spawn((ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                margin: UiRect::top(app::ui::px_p(4.0)),
                ..default()
            },
            background_color: app::ui::MUTE_COLOR.with_a(ACH_COLOR_ALPHA).into(),
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(value_bar_ratio),
                        height: app::ui::px_p(PROGRESS_BAR_H),
                        ..default()
                    },
                    background_color: app::ui::SECONDARY_COLOR.with_a(ACH_COLOR_ALPHA).into(),
                    ..default()
                },
                AchProgressBarValue(String::from(code)),
            ));
        });
}

fn build_ach_running_progress_dots(parent: &mut ChildBuilder, value_total: (u32, u32)) {
    let (value, total) = value_total;
    let remaining = total - value;
    for _ in 0..value {
        parent.spawn(NodeBundle {
            style: Style {
                width: app::ui::px_p(PROGRESS_BAR_H),
                height: app::ui::px_p(PROGRESS_BAR_H),
                ..default()
            },
            background_color: app::ui::SECONDARY_COLOR.with_a(ACH_COLOR_ALPHA).into(),
            ..default()
        });
    }
    for _ in 0..remaining {
        parent.spawn(NodeBundle {
            style: Style {
                width: app::ui::px_p(PROGRESS_BAR_H),
                height: app::ui::px_p(PROGRESS_BAR_H),
                ..default()
            },
            background_color: app::ui::MUTE_COLOR.with_a(ACH_COLOR_ALPHA).into(),
            ..default()
        });
    }
}

struct AchDoneAnimeLens {
    start_right: f32,
    end_right: f32,
}

impl lens::Lens<Style> for AchDoneAnimeLens {
    fn lerp(&mut self, style: &mut Style, ratio: f32) {
        style.right = Val::Px(self.start_right + (self.end_right - self.start_right) * ratio);
    }
}

fn build_ach_done_ui(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, code: &str) {
    let ach_def = app::achievement::fetch_ach_def(code);
    let color = ach_def.color();
    let tween = Tween::new(
        EaseFunction::CubicOut,
        Duration::from_millis(500),
        AchDoneAnimeLens {
            start_right: 36.0,
            end_right: 0.0,
        },
    )
    .then(Delay::new(Duration::from_secs(3)).with_completed_event(ACH_DONE_ANIME_END_EVENT));
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Relative,
                    top: Val::Px(0.0),
                    right: Val::Px(12.0),
                    width: Val::Px(ACH_PANEL_W),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(app::ui::px_p(1.0)),
                    border: UiRect::all(app::ui::px_p(1.0)),
                    ..default()
                },
                background_color: app::ui::BG_COLOR.with_a(ACH_COLOR_ALPHA).into(),
                border_color: color.into(),
                ..default()
            },
            Animator::new(tween),
        ))
        .with_children(|parent| {
            let icon = asset_server.load(ach_def.icon_path());
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(ACH_ICON_SIZE),
                    height: Val::Px(ACH_ICON_SIZE),
                    margin: UiRect::all(app::ui::px_p(3.0)),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
            parent
                .spawn((NodeBundle {
                    style: Style {
                        flex_grow: 1.0,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        margin: UiRect::right(app::ui::px_p(4.0)),
                        ..default()
                    },
                    background_color: app::ui::BG_COLOR.with_a(0.0).into(),
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            ach_def.description(),
                            TextStyle {
                                font: asset_server.load(app::ui::FONT),
                                font_size: ACH_DESC_FS,
                                color: app::ui::SECONDARY_COLOR,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::right(app::ui::px_p(0.0)),
                            ..default()
                        }),
                    );
                    parent.spawn(
                        TextBundle::from_section(
                            ach_def.name(),
                            TextStyle {
                                font: asset_server.load(app::ui::FONT),
                                font_size: ACH_NAME_FS,
                                color,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::right(app::ui::px_p(0.0)),
                            ..default()
                        }),
                    );
                });
        });
}

pub fn build_ach_icon(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, code: &str) {
    let ach_def = app::achievement::fetch_ach_def(code);
    parent
        .spawn((ButtonBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: app::ui::BG_COLOR.into(),
            ..default()
        },))
        .with_children(|parent| {
            let icon = asset_server.load(ach_def.icon_path());
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(ACH_ICON_SIZE),
                    height: Val::Px(ACH_ICON_SIZE),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
        });
}
