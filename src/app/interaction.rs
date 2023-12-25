use crate::app;
use bevy::prelude::*;
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
            )
                .after(NavRequestSystem),
        );
    }
}

fn update_button_interaction(
    mut focusables: Query<(&Focusable, &mut BackgroundColor), (Changed<Focusable>, With<IaButton>)>,
) {
    for (focus, mut color) in focusables.iter_mut() {
        let new_color = if matches!(focus.state(), FocusState::Focused) {
            app::ui::BTN_HOVERED_BG
        } else {
            app::ui::BTN_BG
        };
        *color = new_color.into();
    }
}

fn update_switch_interaction(
    mut focusables: Query<(&Focusable, &mut BorderColor), (Changed<Focusable>, With<IaSwitch>)>,
) {
    for (focus, mut color) in focusables.iter_mut() {
        let new_color = if matches!(focus.state(), FocusState::Focused) {
            app::ui::SECONDARY_COLOR
        } else {
            app::ui::BG_COLOR
        };
        *color = new_color.into();
    }
}

fn update_slider_interaction(
    mut focusables: Query<(&Focusable, &mut BorderColor), (Changed<Focusable>, With<IaSlider>)>,
) {
    for (focus, mut color) in focusables.iter_mut() {
        let new_color = if matches!(focus.state(), FocusState::Focused) {
            app::ui::SECONDARY_COLOR
        } else {
            app::ui::BG_COLOR
        };
        *color = new_color.into();
    }
}

fn update_link_interaction(
    mut focusables: Query<(&Focusable, &mut BorderColor), (Changed<Focusable>, With<IaLink>)>,
) {
    for (focus, mut color) in focusables.iter_mut() {
        let new_color = if matches!(focus.state(), FocusState::Focused) {
            app::ui::FG_COLOR
        } else {
            app::ui::BG_COLOR
        };
        *color = new_color.into();
    }
}
