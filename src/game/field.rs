use super::*;
use crate::app::{self, theme, ui};
use bevy::{ecs::schedule::SystemConfigs, sprite::Anchor, window::WindowResized};
use bevy_prototype_lyon::prelude::*;

pub const FIELD_BAR_H: f32 = 120.0;
pub const FIELD_W: f32 = app::WINDOW_W;
pub const FIELD_H: f32 = app::WINDOW_H;

#[derive(Component)]
pub struct FieldScoreboard(String);

#[derive(Component)]
pub struct TargetRankField(String);

#[derive(Component)]
pub struct TargetValueField(String);

#[derive(Component)]
pub struct TargetBar(String);

#[derive(Component)]
pub struct FieldChainIcon;

#[derive(Component)]
pub struct FieldAchRunning;

#[derive(Component)]
pub struct FieldAchDone;

pub fn field_systems() -> SystemConfigs {
    (
        reset_field_on_window_resize,
        refresh_field,
        update_field_scoreboard,
    )
        .chain()
        .into_configs()
}

const FIELD_PADDING: f32 = (FIELD_BAR_H - FIELD_TEXT_SIZE) / 2.0;
const TARGET_TEXT_SIZE: f32 = ui::FONT_SIZE * 0.8;
const TARGET_COLOR_ALPHA: f32 = 0.2;
const TARGET_COLOR: Color = Color::rgba(0.5, 0.5, 0.5, TARGET_COLOR_ALPHA);
const TARGET_BG_COLOR: Color = Color::rgba(0.5, 0.5, 0.5, TARGET_COLOR_ALPHA * 0.5);

fn reset_field_on_window_resize(
    mut resize_events: EventReader<WindowResized>,
    mut game_status: ResMut<GameStatus>,
) {
    for _event in resize_events.read() {
        game_status.is_refresh_required = true;
    }
}

fn refresh_field(
    mut commands: Commands,
    bg_query: Query<Entity, With<GameBg>>,
    dyn_query: Query<Entity, With<GameDyn>>,
    cover_query: Query<Entity, With<GameCover>>,
    window_query: Query<&Window>,
    mut refresh_timer: ResMut<timer::GameRefreshTimer>,
    mut game_status: ResMut<GameStatus>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut gizmos: Gizmos,
) {
    // TODO: Test
    gizmos.circle_2d(Vec2::ZERO, 1.0, Color::RED);

    if game_status.is_refresh_required {
        let bg_entity = bg_query.get_single().unwrap();
        let mut entity_commands = commands.get_entity(bg_entity).unwrap();
        entity_commands.despawn_descendants();
        entity_commands.with_children(|parent| {
            build_reactor_field(parent, &asset_server, &game_status);
        });
        let cover_entity = cover_query.get_single().unwrap();
        let mut entity_commands = commands.get_entity(cover_entity).unwrap();
        entity_commands.despawn_descendants();
        if game_status.is_cover_on() {
            entity_commands.with_children(|parent| {
                draw_reactor_cover(parent, &window_query);
            });
        }
        game_status.is_refresh_required = false;
    }
    let dyn_entity = dyn_query.get_single().unwrap();
    if refresh_timer.0.tick(time.delta()).just_finished() {
        let mut entity_commands = commands.get_entity(dyn_entity).unwrap();
        entity_commands.despawn_descendants();
    }
}

fn update_field_scoreboard(
    mut scoreboard_query: Query<(&mut Text, &FieldScoreboard), With<FieldScoreboard>>,
    mut scoreboard_chain_icon_query: Query<&mut Handle<Image>, With<FieldChainIcon>>,
    mut status: ResMut<GameStatus>,
    asset_server: Res<AssetServer>,
    mut scoreboard_timer: ResMut<timer::GameScoreboardTimer>,
    time: Res<Time>,
) {
    if scoreboard_timer.0.tick(time.delta()).just_finished() {
        status.increase("time", 1);
        for (mut text, field) in scoreboard_query.iter_mut() {
            let name = field.0.as_ref();
            text.sections[0].value = format_field_text(name, status.fetch(name));
        }
        let mut image = scoreboard_chain_icon_query.single_mut();
        *image = match status.current_chain() {
            status::StatusChain::Control => asset_server.load("images/icons/square.png"),
            status::StatusChain::None => asset_server.load("images/icons/circle.png"),
            status::StatusChain::Hyper => asset_server.load("images/icons/hexagon.png"),
        };
    }
}

const FIELD_NAMES: [&str; 4] = ["score", "time", "alpha_count", "chain_length"];
const FIELD_LINE_W: f32 = ui::SPACE_SIZE * 0.5;
const FIELD_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
const FIELD_TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
const FIELD_TEXT_SIZE: f32 = 42.0;
const FIELD_FOOTER_SB_H: f32 = 84.0;
const FIELD_FOOTER_ICON_SIZE: f32 = 48.0;
const FIELD_FOOTER_TEXT_H_BIAS: f32 = -5.0;
const FIELD_FOOTER_ENTRY_GAP: f32 = 24.0;
const FIELD_P: f32 = 12.0;
const FIELD_ACH_W: f32 = 380.0;
const FIELD_ACH_H: f32 = FIELD_BAR_H - FIELD_P * 2.0;

fn build_reactor_field(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    game_status: &ResMut<GameStatus>,
) {
    // NOTE: Draw field
    let rect = shapes::Rectangle {
        extents: Vec2::new(FIELD_W, FIELD_H),
        ..default()
    };
    let geo_builder = GeometryBuilder::new().add(&rect);
    parent.spawn((
        ShapeBundle {
            path: geo_builder.build(),
            spatial: SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.001),
                ..default()
            },
            ..default()
        },
        Stroke::new(FIELD_COLOR, FIELD_LINE_W),
    ));
    let mut path_builder = PathBuilder::new();
    let half_field_w = FIELD_W / 2.0;
    let half_field_h = FIELD_H / 2.0;
    let header_y = half_field_h - FIELD_BAR_H;
    let footer_y = -half_field_h + FIELD_BAR_H;
    path_builder.move_to(Vec2::new(-half_field_w, header_y));
    path_builder.line_to(Vec2::new(half_field_w, header_y));
    path_builder.move_to(Vec2::new(-half_field_w, footer_y));
    path_builder.line_to(Vec2::new(half_field_w, footer_y));
    parent.spawn((
        ShapeBundle {
            path: path_builder.build(),
            spatial: SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.002),
                ..default()
            },
            ..default()
        },
        Stroke::new(FIELD_COLOR, FIELD_LINE_W),
    ));

    // NOTE: Draw header
    let header_cy = half_field_h - FIELD_BAR_H / 2.0;
    parent.spawn(SpriteBundle {
        texture: asset_server.load("images/game/title.png"),
        transform: Transform::from_xyz(0.0, header_cy, 0.003),
        ..default()
    });
    let center_pos = Vec2::new(-half_field_w + FIELD_P + FIELD_ACH_W / 2.0, header_cy);
    draw_ach_panel(
        parent,
        asset_server,
        game_status,
        center_pos,
        "not_moving_xxx_s",
    );
    let center_pos = Vec2::new(half_field_w - FIELD_P - FIELD_ACH_W / 2.0, header_cy);
    draw_ach_panel(parent, asset_server, game_status, center_pos, "max_c_xxx");
    let center_pos = Vec2::new(0.0, header_cy);
    draw_ach_panel(parent, asset_server, game_status, center_pos, "score_xxx");

    // NOTE: Draw footer
    let footer_cy = -half_field_h + FIELD_FOOTER_SB_H / 2.0;
    let mut footer_entry_start_x = -half_field_w + FIELD_FOOTER_SB_H + FIELD_FOOTER_ENTRY_GAP;
    for name in FIELD_NAMES.iter() {
        let icon_path = match *name {
            "score" => "images/icons/trophy-fill.png",
            "time" => "images/icons/timer-fill.png",
            "alpha_count" => "images/icons/circles-three-fill.png",
            "chain_length" => "images/icons/line-segments.png",
            _ => panic!("Invalid field"),
        };
        parent.spawn(SpriteBundle {
            texture: asset_server.load(icon_path),
            transform: Transform::from_xyz(footer_entry_start_x, footer_cy, 0.004),
            sprite: Sprite {
                anchor: Anchor::CenterLeft,
                ..default()
            },
            ..default()
        });
        footer_entry_start_x += FIELD_FOOTER_ICON_SIZE;
        let text_w = match *name {
            "score" | "time" => 260.0,
            "alpha_count" | "chain_length" => 160.0,
            _ => panic!("Invalid field"),
        };
        parent.spawn((
            Text2dBundle {
                text: Text::from_section(
                    format_field_text(name, 0),
                    TextStyle {
                        font: asset_server.load(theme::FONT_DIGIT),
                        font_size: FIELD_TEXT_SIZE,
                        color: FIELD_TEXT_COLOR,
                    },
                )
                .with_alignment(TextAlignment::Right),
                transform: Transform::from_xyz(
                    footer_entry_start_x + text_w / 2.0,
                    footer_cy + FIELD_FOOTER_TEXT_H_BIAS,
                    0.004,
                ),
                ..default()
            },
            FieldScoreboard(String::from(*name)),
        ));
        footer_entry_start_x += text_w + FIELD_FOOTER_ENTRY_GAP;
    }
    footer_entry_start_x += -FIELD_FOOTER_ENTRY_GAP;
    parent.spawn((
        SpriteBundle {
            texture: asset_server.load("images/icons/circle.png"),
            transform: Transform::from_xyz(footer_entry_start_x, footer_cy, 0.004),
            sprite: Sprite {
                anchor: Anchor::CenterLeft,
                ..default()
            },
            ..default()
        },
        FieldChainIcon,
    ));
}

const FIELD_COVER_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.95);

fn draw_reactor_cover(parent: &mut ChildBuilder, window_query: &Query<&Window>) {
    let window = window_query.get_single().unwrap();
    let win_w = window.resolution.width();
    let win_h = window.resolution.height();
    let rect = shapes::Rectangle {
        extents: Vec2::new(win_w, win_h),
        ..default()
    };
    let geo_builder = GeometryBuilder::new().add(&rect);
    parent.spawn((
        ShapeBundle {
            path: geo_builder.build(),
            ..default()
        },
        Fill::color(FIELD_COVER_COLOR),
    ));
}

const ACH_ICON_SIZE: f32 = app::ui::FONT_SIZE * 2.0;
const ACH_DESC_FS: f32 = app::ui::FONT_SIZE * 0.8;
const ACH_NAME_FS: f32 = app::ui::FONT_SIZE * 1.2;
const ACH_P: f32 = 12.0;

fn draw_ach_panel(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    game_status: &ResMut<GameStatus>,
    center_pos: Vec2,
    code: &str,
) {
    let ach = fetch_ach_def(code);
    let rect = shapes::Rectangle {
        extents: Vec2::new(FIELD_ACH_W, FIELD_ACH_H),
        ..default()
    };
    let geo_builder = GeometryBuilder::new().add(&rect);
    parent.spawn((
        ShapeBundle {
            path: geo_builder.build(),
            spatial: SpatialBundle {
                transform: Transform::from_xyz(center_pos.x, center_pos.y, 0.003),
                ..default()
            },
            ..default()
        },
        Stroke::new(FIELD_COLOR, FIELD_LINE_W),
    ));
    let text_pos = center_pos
        + Vec2::new(
            -FIELD_ACH_W / 2.0 + ACH_P * 2.0 + ACH_ICON_SIZE,
            FIELD_ACH_H / 2.0 - ACH_P,
        );
    parent.spawn((Text2dBundle {
        text: Text::from_section(
            ach.description(),
            TextStyle {
                font: asset_server.load(theme::FONT),
                font_size: ACH_DESC_FS,
                color: FIELD_TEXT_COLOR,
            },
        ),
        text_anchor: Anchor::TopLeft,
        transform: Transform::from_xyz(text_pos.x, text_pos.y, 0.004),
        ..default()
    },));
    match ach.progress_ui() {
        AchievementProgressUi::Dots => {}
        AchievementProgressUi::Bar => {}
    }
}
