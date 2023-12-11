use bevy::prelude::*;

pub const MENU_W: f32 = 300.0;
pub const FONT_SIZE: f32 = 32.0;
pub const BTN_FS: f32 = FONT_SIZE;
pub const SPACE_SIZE: f32 = FONT_SIZE / 10.0;
pub const BORDER_W: f32 = SPACE_SIZE * 1.0;
pub const ICON_SIZE: f32 = FONT_SIZE * 0.8;
pub const BTN_PADDING: f32 = 3.0;

pub const BG_COLOR: Color = Color::BLACK;
pub const FG_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const SECONDARY_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
pub const MUTE_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
pub const COVER_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.9);
pub const BTN_BG: Color = Color::rgb(0.15, 0.15, 0.15);
pub const BTN_HOVERED_BG: Color = Color::rgb(0.25, 0.25, 0.25);
pub const BTN_PRESSED_BG: Color = Color::rgb(0.45, 0.45, 0.45);

pub const FONT: &str = "fonts/SYNNova-Regular.otf";
pub const FONT_DIGIT: &str = "fonts/slkscr.ttf";
pub const FONT_HW: &str = "fonts/VAG-HandWritten.otf";

pub fn px_p(size: f32) -> Val {
    Val::Px(SPACE_SIZE * size)
}

pub fn build_icon_btn(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    bundle: impl Bundle,
    style: Style,
    icon: &str,
) -> Entity {
    let icon_style = Style {
        width: Val::Auto,
        height: Val::Auto,
        padding: UiRect::all(px_p(BTN_PADDING)),
        ..style
    };
    build_btn(parent, asset_server, bundle, icon_style, None, Some(icon))
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

const MENU_ENTRY_W: f32 = 280.0;
const MENU_ENTRY_PADDING: f32 = 20.0;
const MENU_ENTRY_RATIO: f32 = 1.2;

pub fn build_menu_entry(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    bundle: impl Bundle,
    text: &str,
    icon: &str,
) -> Entity {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(MENU_ENTRY_W),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(MENU_ENTRY_PADDING)),
                    padding: UiRect::all(px_p(4.0)),
                    ..default()
                },
                background_color: BG_COLOR.into(),
                ..default()
            },
            bundle,
        ))
        .with_children(|parent| {
            let icon_path = format!("images/icons/{}.png", icon);
            let icon = asset_server.load(icon_path);
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(ICON_SIZE * MENU_ENTRY_RATIO),
                    height: Val::Px(ICON_SIZE * MENU_ENTRY_RATIO),
                    margin: UiRect::right(px_p(3.0)),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
            parent.spawn(
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load(FONT),
                        font_size: FONT_SIZE * MENU_ENTRY_RATIO,
                        color: FG_COLOR,
                    },
                )
                .with_style(Style {
                    margin: UiRect::right(px_p(2.0)),
                    ..default()
                }),
            );
        })
        .id()
}

pub struct ButtonInteractionPlugin;

#[derive(Component)]
pub struct LinkButton;

#[derive(Component)]
pub struct SwitchButton;

#[derive(Component)]
pub struct RangeButton;

impl Plugin for ButtonInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_interaction);
    }
}

fn button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            (
                With<Button>,
                Without<LinkButton>,
                Without<SwitchButton>,
                Without<RangeButton>,
            ),
        ),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        *color = match *interaction {
            Interaction::Pressed => BTN_PRESSED_BG.into(),
            Interaction::Hovered => BTN_HOVERED_BG.into(),
            Interaction::None => BTN_BG.into(),
        }
    }
}
