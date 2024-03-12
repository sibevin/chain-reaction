use crate::{app, app::theme::*};
use bevy::prelude::*;
use bevy_ui_navigation::prelude::*;

mod plugin;
mod timer;

pub use plugin::AppUiPlugin;

pub const FONT_SIZE: f32 = 36.0;
pub const BTN_FS: f32 = FONT_SIZE;
pub const SPACE_SIZE: f32 = FONT_SIZE / 10.0;
pub const BORDER_W: f32 = SPACE_SIZE * 1.0;
pub const ICON_SIZE: f32 = FONT_SIZE * 0.8;
pub const BTN_PADDING: f32 = 5.0;
pub const MENU_ENTRY_PADDING: f32 = 3.0;
pub const PAGE_PADDING: f32 = 3.0;

pub fn px_p(size: f32) -> Val {
    Val::Px(SPACE_SIZE * size)
}

pub fn fs_x(scale: f32) -> f32 {
    FONT_SIZE * scale
}

pub fn despawn_ui<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn build_btn(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    bundle: impl Bundle,
    style: Style,
    text: Option<&str>,
    icon: Option<&str>,
) -> Entity {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    height: Val::Auto,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..style
                },
                background_color: BTN_BG.into(),
                ..default()
            },
            bundle,
        ))
        .with_children(|parent| {
            let gap = if icon.is_some() && text.is_some() {
                px_p(6.0)
            } else {
                px_p(0.0)
            };
            if let Some(icon) = icon {
                let icon_path = format!("images/icons/{}.png", icon);
                let icon = asset_server.load(icon_path);
                parent.spawn(ImageBundle {
                    style: Style {
                        width: Val::Px(ICON_SIZE),
                        height: Val::Px(ICON_SIZE),
                        margin: UiRect::right(gap),
                        ..default()
                    },
                    image: UiImage::new(icon),
                    ..default()
                });
            }
            if let Some(text) = text {
                parent.spawn(TextBundle::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load(FONT),
                        font_size: BTN_FS,
                        color: FG_COLOR,
                    },
                ));
            }
        })
        .id()
}

pub fn build_icon_btn(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    bundle: impl Bundle,
    style: Style,
    icon: &str,
) -> Entity {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Auto,
                    height: Val::Auto,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(px_p(BTN_PADDING * 0.6)),
                    ..style
                },
                background_color: BTN_BG.into(),
                ..default()
            },
            bundle,
        ))
        .with_children(|parent| {
            let icon_path = format!("images/icons/{}.png", icon);
            let icon = asset_server.load(icon_path);
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(ICON_SIZE * 1.5),
                    height: Val::Px(ICON_SIZE * 1.5),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
        })
        .id()
}

pub fn build_link(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    bundle: impl Bundle,
    text: &str,
    icon: Option<&str>,
    font: &str,
    enable_interaction: bool,
) -> Entity {
    let mut entity = parent.spawn((
        NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::vertical(px_p(1.0)),
                padding: UiRect::all(px_p(1.0)),
                ..default()
            },
            background_color: LINK_BG.into(),
            ..default()
        },
        bundle,
    ));
    entity.with_children(|parent| {
        if let Some(icon) = icon {
            let icon_path = format!("images/icons/{}.png", icon);
            let icon = asset_server.load(icon_path);
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(ICON_SIZE),
                    height: Val::Px(ICON_SIZE),
                    margin: UiRect::right(px_p(4.0)),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
        }
        let font = if font == "main" {
            FONT
        } else if font == "digit" {
            FONT_DIGIT
        } else if font == "hw" {
            FONT_HW
        } else {
            FONT
        };
        parent.spawn(
            TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_server.load(font),
                    font_size: FONT_SIZE,
                    color: FG_COLOR,
                },
            )
            .with_style(Style {
                margin: UiRect::right(px_p(2.0)),
                ..default()
            }),
        );
    });
    if enable_interaction {
        entity.insert((app::interaction::IaLink, Focusable::default()));
    };
    entity.id()
}

#[derive(Component)]
pub struct RangeButton;
