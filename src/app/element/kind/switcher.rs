use super::*;
use crate::app::*;
use bevy::input;

const SWITCH_ICON_RATIO: f32 = 1.6;

pub fn build_element(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    bundle: impl Bundle,
    data: ElementTargetValuePair,
) -> Entity {
    let value = data.bool_value.unwrap();
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::new(
                        ui::px_p(6.0),
                        ui::px_p(6.0),
                        ui::px_p(3.0),
                        ui::px_p(3.0),
                    ),
                    ..default()
                },
                background_color: theme::BTN_BG.into(),
                ..default()
            },
            bundle,
            interaction::IaSwitch,
            ElementData::Switcher { data },
            Focusable::default(),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "ON",
                TextStyle {
                    font: asset_server.load(theme::FONT),
                    font_size: ui::FONT_SIZE,
                    color: theme::FG_COLOR,
                },
            ));
            let icon = if value {
                asset_server.load("images/icons/toggle-left-fill.png")
            } else {
                asset_server.load("images/icons/toggle-right-fill.png")
            };
            parent.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Px(ui::ICON_SIZE * SWITCH_ICON_RATIO),
                        height: Val::Px(ui::ICON_SIZE * SWITCH_ICON_RATIO),
                        margin: UiRect::horizontal(ui::px_p(6.0)),
                        ..default()
                    },
                    image: UiImage::new(icon),
                    ..default()
                },
                ElementImage,
            ));
            parent.spawn(TextBundle::from_section(
                "OFF",
                TextStyle {
                    font: asset_server.load(theme::FONT),
                    font_size: ui::FONT_SIZE,
                    color: theme::MUTE_COLOR,
                },
            ));
        })
        .id()
}

pub fn update_ui_image(
    ele_image: &mut UiImage,
    data: &ElementTargetValuePair,
    asset_server: &Res<AssetServer>,
) {
    let value = data.bool_value.unwrap();
    ele_image.texture = if value {
        asset_server.load("images/icons/toggle-left-fill.png")
    } else {
        asset_server.load("images/icons/toggle-right-fill.png")
    };
}

pub fn toggle_switcher(data: &mut ElementTargetValuePair) {
    data.bool_value = Some(!data.bool_value.unwrap());
}

pub fn handle_mouse_dragging(
    motion_events: &mut EventReader<input::mouse::MouseMotion>,
    data: &mut ElementTargetValuePair,
) {
    let motion_events = motion_events.read().collect::<Vec<_>>();
    if let Some(motion_event) = motion_events.iter().rev().take(3).next() {
        if motion_event.delta.x.abs() > 3.0 {
            data.bool_value = Some(motion_event.delta.x < 0.0);
        }
    }
}
