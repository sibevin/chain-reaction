use crate::app;
use bevy::prelude::*;
use bevy_persistent::prelude::*;
use bevy_ui_navigation::{
    prelude::{FocusState, Focusable},
    NavRequestSystem,
};

#[derive(Component)]
pub struct IaButton;

#[derive(Component)]
pub struct IaSwitch;

#[derive(Component)]
pub struct IaSlider;

#[derive(Component)]
pub struct IaLink;

#[derive(Component)]
pub struct IaPanel;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_button_interaction,
                update_switch_interaction,
                update_slider_interaction,
                update_link_interaction,
                update_panel_interaction,
            )
                .after(NavRequestSystem),
        );
    }
}

type FocusableButton = (Changed<Focusable>, With<IaButton>);

fn update_button_interaction(
    mut focusables: Query<(&Focusable, &mut BackgroundColor), FocusableButton>,
    mut commands: Commands,
    audio_se_asset: Res<app::audio::AudioSeAsset>,
    settings: Res<Persistent<app::settings::Settings>>,
) {
    for (focus, mut color) in focusables.iter_mut() {
        let new_color = if matches!(focus.state(), FocusState::Focused) {
            app::audio::play_se(
                app::audio::AudioSe::Focus,
                &mut commands,
                &audio_se_asset,
                settings.as_ref(),
            );
            app::ui::BTN_HOVERED_BG
        } else {
            app::ui::BTN_BG
        };
        *color = new_color.into();
    }
}

type FocusableSwitch = (Changed<Focusable>, With<IaSwitch>);

fn update_switch_interaction(
    mut focusables: Query<(&Focusable, &mut BorderColor), FocusableSwitch>,
    mut commands: Commands,
    audio_se_asset: Res<app::audio::AudioSeAsset>,
    settings: Res<Persistent<app::settings::Settings>>,
) {
    for (focus, mut color) in focusables.iter_mut() {
        let new_color = if matches!(focus.state(), FocusState::Focused) {
            app::audio::play_se(
                app::audio::AudioSe::Focus,
                &mut commands,
                &audio_se_asset,
                settings.as_ref(),
            );
            app::ui::SECONDARY_COLOR
        } else {
            app::ui::BG_COLOR
        };
        *color = new_color.into();
    }
}

type FocusableSlider = (Changed<Focusable>, With<IaSlider>);

fn update_slider_interaction(
    mut focusables: Query<(&Focusable, &mut BorderColor), FocusableSlider>,
    mut commands: Commands,
    audio_se_asset: Res<app::audio::AudioSeAsset>,
    settings: Res<Persistent<app::settings::Settings>>,
) {
    for (focus, mut color) in focusables.iter_mut() {
        let new_color = if matches!(focus.state(), FocusState::Focused) {
            app::audio::play_se(
                app::audio::AudioSe::Focus,
                &mut commands,
                &audio_se_asset,
                settings.as_ref(),
            );
            app::ui::SECONDARY_COLOR
        } else {
            app::ui::BG_COLOR
        };
        *color = new_color.into();
    }
}

type FocusableLink = (Changed<Focusable>, With<IaLink>);

fn update_link_interaction(
    mut focusables: Query<(&Focusable, &mut BorderColor), FocusableLink>,
    mut commands: Commands,
    audio_se_asset: Res<app::audio::AudioSeAsset>,
    settings: Res<Persistent<app::settings::Settings>>,
) {
    for (focus, mut color) in focusables.iter_mut() {
        let new_color = if matches!(focus.state(), FocusState::Focused) {
            app::audio::play_se(
                app::audio::AudioSe::Focus,
                &mut commands,
                &audio_se_asset,
                settings.as_ref(),
            );
            app::ui::FG_COLOR
        } else {
            app::ui::BG_COLOR
        };
        *color = new_color.into();
    }
}

type FocusablePanel = (Changed<Focusable>, With<IaPanel>);

fn update_panel_interaction(mut focusables: Query<(&Focusable, &mut BorderColor), FocusablePanel>) {
    for (focus, mut color) in focusables.iter_mut() {
        let new_color = if matches!(focus.state(), FocusState::Focused) {
            app::ui::FG_COLOR
        } else {
            app::ui::MUTE_COLOR
        };
        *color = new_color.into();
    }
}
