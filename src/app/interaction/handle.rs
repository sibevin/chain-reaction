use super::*;
use crate::app::{audio, settings};
use bevy_persistent::prelude::*;

const SHOOT_W: f32 = ui::FONT_SIZE * 0.1;

#[derive(Component)]
pub struct IaButton;

#[derive(Component)]
pub struct IaMenuEntry;

#[derive(Component)]
pub struct IaSwitch;

#[derive(Component)]
pub struct IaSlider;

#[derive(Component)]
pub struct IaLink;

#[derive(Component)]
pub struct IaCrossPanel;

#[derive(Component)]
pub struct IaAnimeEffect;

#[derive(Default)]
struct FocusTarget {
    pub pos: Vec2,
    pub size: Vec2,
}

type FocusableButton = (Changed<Focusable>, With<IaButton>);

pub fn handle_button_interaction(
    mut commands: Commands,
    mut focusables: Query<(&Focusable, &GlobalTransform, &Node), FocusableButton>,
    ae_query: Query<Entity, With<IaAnimeEffect>>,
    mut ae_status: ResMut<AnimeEffectStatus>,
    window: Query<&Window>,
    settings: Res<Persistent<settings::Settings>>,
    asset_server: Res<AssetServer>,
) {
    let mut target: Option<FocusTarget> = None;
    for (focus, g_trans, node) in focusables.iter_mut() {
        if matches!(focus.state(), FocusState::Focused) {
            target = Some(fetch_focus_target(&window, g_trans, node));
        } else {
            for ae_entity in ae_query.iter() {
                despawn_anime_effect(ae_entity, &mut ae_status);
            }
        }
    }
    if let Some(target) = target {
        if target.size.x > target.size.y * 1.5 {
            draw_shoot_line(&mut commands, target, Vec2::ZERO);
        } else {
            draw_shoot_circle(&mut commands, target, Vec2::ZERO);
        }
        audio::play_se("focus", &mut commands, &asset_server, settings.as_ref());
    }
}

type FocusableMenuEntry = (Changed<Focusable>, With<IaMenuEntry>);

pub fn handle_menu_entry_interaction(
    mut commands: Commands,
    mut focusables: Query<(&Focusable, &GlobalTransform, &Node), FocusableMenuEntry>,
    ae_query: Query<Entity, With<IaAnimeEffect>>,
    mut ae_status: ResMut<AnimeEffectStatus>,
    window: Query<&Window>,
    settings: Res<Persistent<settings::Settings>>,
    asset_server: Res<AssetServer>,
) {
    let mut target: Option<FocusTarget> = None;
    for (focus, g_trans, node) in focusables.iter_mut() {
        if matches!(focus.state(), FocusState::Focused) {
            target = Some(fetch_focus_target(&window, g_trans, node));
        } else {
            for ae_entity in ae_query.iter() {
                despawn_anime_effect(ae_entity, &mut ae_status);
            }
        }
    }
    if let Some(target) = target {
        draw_shoot_rect(&mut commands, target, Vec2::ZERO);
        audio::play_se("focus", &mut commands, &asset_server, settings.as_ref());
    }
}

type FocusableSwitch = (Changed<Focusable>, With<IaSwitch>);

pub fn handle_switch_interaction(
    mut commands: Commands,
    mut focusables: Query<(&Focusable, &GlobalTransform, &Node), FocusableSwitch>,
    ae_query: Query<Entity, With<IaAnimeEffect>>,
    mut ae_status: ResMut<AnimeEffectStatus>,
    window: Query<&Window>,
    settings: Res<Persistent<settings::Settings>>,
    asset_server: Res<AssetServer>,
) {
    let mut target: Option<FocusTarget> = None;
    for (focus, g_trans, node) in focusables.iter_mut() {
        if matches!(focus.state(), FocusState::Focused) {
            target = Some(fetch_focus_target(&window, g_trans, node));
        } else {
            for ae_entity in ae_query.iter() {
                despawn_anime_effect(ae_entity, &mut ae_status);
            }
        }
    }
    if let Some(target) = target {
        draw_shoot_rect(&mut commands, target, Vec2::ZERO);
        audio::play_se("focus", &mut commands, &asset_server, settings.as_ref());
    }
}

type FocusableSlider = (Changed<Focusable>, With<IaSlider>);

pub fn handle_slider_interaction(
    mut commands: Commands,
    mut focusables: Query<(&Focusable, &GlobalTransform, &Node), FocusableSlider>,
    ae_query: Query<Entity, With<IaAnimeEffect>>,
    mut ae_status: ResMut<AnimeEffectStatus>,
    window: Query<&Window>,
    settings: Res<Persistent<settings::Settings>>,
    asset_server: Res<AssetServer>,
) {
    let mut target: Option<FocusTarget> = None;
    for (focus, g_trans, node) in focusables.iter_mut() {
        if matches!(focus.state(), FocusState::Focused) {
            target = Some(fetch_focus_target(&window, g_trans, node));
        } else {
            for ae_entity in ae_query.iter() {
                despawn_anime_effect(ae_entity, &mut ae_status);
            }
        }
    }
    if let Some(target) = target {
        draw_shoot_rect(&mut commands, target, Vec2::ZERO);
        audio::play_se("focus", &mut commands, &asset_server, settings.as_ref());
    }
}

type FocusableLink = (Changed<Focusable>, With<IaLink>);

pub fn handle_link_interaction(
    mut commands: Commands,
    mut focusables: Query<(&Focusable, &GlobalTransform, &Node), FocusableLink>,
    ae_query: Query<Entity, With<IaAnimeEffect>>,
    mut ae_status: ResMut<AnimeEffectStatus>,
    window: Query<&Window>,
    settings: Res<Persistent<settings::Settings>>,
    asset_server: Res<AssetServer>,
) {
    let mut target: Option<FocusTarget> = None;
    for (focus, g_trans, node) in focusables.iter_mut() {
        if matches!(focus.state(), FocusState::Focused) {
            target = Some(fetch_focus_target(&window, g_trans, node));
        } else {
            for ae_entity in ae_query.iter() {
                despawn_anime_effect(ae_entity, &mut ae_status);
            }
        }
    }
    if let Some(target) = target {
        draw_shoot_line(
            &mut commands,
            target,
            Vec2::new(ui::fs_x(0.3), ui::fs_x(0.3)),
        );
        audio::play_se("focus", &mut commands, &asset_server, settings.as_ref());
    }
}

type FocusableCrossPanel = (Changed<Focusable>, With<IaCrossPanel>);

const CROSS_PANEL_P: f32 = ui::FONT_SIZE * -0.3;

pub fn handle_cross_panel_interaction(
    mut commands: Commands,
    mut focusables: Query<(&Focusable, &GlobalTransform, &Node), FocusableCrossPanel>,
    ae_query: Query<Entity, With<IaAnimeEffect>>,
    mut ae_status: ResMut<AnimeEffectStatus>,
    window: Query<&Window>,
    settings: Res<Persistent<settings::Settings>>,
    asset_server: Res<AssetServer>,
) {
    let mut target: Option<FocusTarget> = None;
    for (focus, g_trans, node) in focusables.iter_mut() {
        if matches!(focus.state(), FocusState::Focused) {
            target = Some(fetch_focus_target(&window, g_trans, node));
        } else {
            for ae_entity in ae_query.iter() {
                despawn_anime_effect(ae_entity, &mut ae_status);
            }
        }
    }
    if let Some(target) = target {
        draw_shoot_rect(&mut commands, target, Vec2::ONE * CROSS_PANEL_P);
        audio::play_se("focus", &mut commands, &asset_server, settings.as_ref());
    }
}

fn fetch_focus_target(
    window: &Query<&Window>,
    g_trans: &GlobalTransform,
    node: &Node,
) -> FocusTarget {
    let g_pos = Vec2::new(g_trans.translation().x, -g_trans.translation().y);
    let window = window.single();
    let win_w = window.resolution.width();
    let win_h = window.resolution.height();
    FocusTarget {
        pos: g_pos - Vec2::new(win_w / 2.0, -win_h / 2.0),
        size: Vec2::new(node.size().x, node.size().y),
    }
}

fn draw_shoot_line(commands: &mut Commands, target: FocusTarget, padding: Vec2) {
    if target.size.x > 0.0 && target.size.y > 0.0 {
        let size_x = target.size.x + padding.x;
        let size_y = target.size.y + padding.y;
        insert_anime_effect(
            commands,
            AnimeEffectParam {
                kind: AnimeEffectKind::LineShoot,
                color: HIGHLIGHT_COLOR,
                pos_1: Vec2::new(target.pos.x - size_x / 2.0, target.pos.y - size_y / 2.0),
                pos_2: Vec2::new(target.pos.x + size_x / 2.0, target.pos.y - size_y / 2.0),
                width_start: SHOOT_W,
                width_end: SHOOT_W,
            },
            IaAnimeEffect,
        );
    }
}

fn draw_shoot_circle(commands: &mut Commands, target: FocusTarget, padding: Vec2) {
    if target.size.x > 0.0 && target.size.y > 0.0 {
        insert_anime_effect(
            commands,
            AnimeEffectParam {
                kind: AnimeEffectKind::CircleShoot,
                color: HIGHLIGHT_COLOR,
                pos_1: Vec2::new(target.pos.x, target.pos.y),
                pos_2: Vec2::new(
                    target.pos.x + target.size.x / 2.0 + padding.x,
                    target.pos.y + target.size.y / 2.0 + padding.y,
                ),
                width_start: SHOOT_W,
                width_end: SHOOT_W,
            },
            IaAnimeEffect,
        );
    }
}

fn draw_shoot_rect(commands: &mut Commands, target: FocusTarget, padding: Vec2) {
    if target.size.x > 0.0 && target.size.y > 0.0 {
        let size_x = target.size.x + padding.x;
        let size_y = target.size.y + padding.y;
        insert_anime_effect(
            commands,
            AnimeEffectParam {
                kind: AnimeEffectKind::RectShoot,
                color: HIGHLIGHT_COLOR,
                pos_1: Vec2::new(target.pos.x - size_x / 2.0, target.pos.y + size_y / 2.0),
                pos_2: Vec2::new(target.pos.x + size_x / 2.0, target.pos.y - size_y / 2.0),
                width_start: SHOOT_W,
                width_end: SHOOT_W,
            },
            IaAnimeEffect,
        );
    }
}
