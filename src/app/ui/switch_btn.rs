use crate::app::ui::*;

#[derive(Component)]
pub struct SwitchButton(bool);

const SWITCH_ICON_RATIO: f32 = 1.6;

pub fn build_switch_btn(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    bundle: impl Bundle,
    init_value: bool,
) -> Entity {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::new(px_p(6.0), px_p(6.0), px_p(3.0), px_p(3.0)),
                    ..default()
                },
                background_color: BTN_BG.into(),
                ..default()
            },
            bundle,
            app::interaction::IaSwitch,
            Focusable::default(),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "ON",
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: FONT_SIZE,
                    color: FG_COLOR,
                },
            ));
            let icon = if init_value {
                asset_server.load("images/icons/toggle-left-fill.png")
            } else {
                asset_server.load("images/icons/toggle-right-fill.png")
            };
            parent.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Px(ICON_SIZE * SWITCH_ICON_RATIO),
                        height: Val::Px(ICON_SIZE * SWITCH_ICON_RATIO),
                        margin: UiRect::horizontal(px_p(6.0)),
                        ..default()
                    },
                    image: UiImage::new(icon),
                    ..default()
                },
                SwitchButton(init_value),
            ));
            parent.spawn(TextBundle::from_section(
                "OFF",
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: FONT_SIZE,
                    color: MUTE_COLOR,
                },
            ));
        })
        .id()
}

pub fn update_switch_btn_value(
    entity: Entity,
    switch_btn_query: &mut Query<(&Parent, &mut UiImage, &mut SwitchButton)>,
    asset_server: &Res<AssetServer>,
    value: bool,
) {
    for (parent, mut icon_image, mut sbi) in switch_btn_query.iter_mut() {
        if parent.get() == entity {
            sbi.0 = value;
            icon_image.texture = if value {
                asset_server.load("images/icons/toggle-left-fill.png")
            } else {
                asset_server.load("images/icons/toggle-right-fill.png")
            };
        }
    }
}

pub fn update_switch_btn_display(
    children: &Children,
    switch_btn_query: &mut Query<(Entity, &mut UiImage, &mut SwitchButton)>,
    asset_server: &Res<AssetServer>,
) -> Option<bool> {
    for (icon_entity, mut icon_image, mut sbi) in switch_btn_query.iter_mut() {
        for child in children {
            if *child == icon_entity {
                sbi.0 = !sbi.0;
                icon_image.texture = if sbi.0 {
                    asset_server.load("images/icons/toggle-left-fill.png")
                } else {
                    asset_server.load("images/icons/toggle-right-fill.png")
                };
                return Some(sbi.0);
            }
        }
    }
    return None;
}
