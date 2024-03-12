use super::*;
use crate::app::{theme, ui};

const MENU_ENTRY_W: f32 = ui::FONT_SIZE * 8.0;
const MENU_ENTRY_B: f32 = ui::FONT_SIZE * 0.01;
const MENU_ENTRY_RATIO: f32 = 1.2;

pub fn build_element(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    bundle: impl Bundle,
    icon: &str,
    text: &str,
) -> Entity {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(MENU_ENTRY_W),
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    column_gap: ui::px_p(4.0),
                    padding: UiRect::all(ui::px_p(4.0)),
                    ..default()
                },
                background_color: theme::BTN_BG.into(),
                ..default()
            },
            bundle,
            ElementData::MenuEntry,
        ))
        .with_children(|parent| {
            let icon_path = format!("images/icons/{}.png", icon);
            let icon = asset_server.load(icon_path);
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(ui::ICON_SIZE * MENU_ENTRY_RATIO),
                    height: Val::Px(ui::ICON_SIZE * MENU_ENTRY_RATIO),
                    margin: UiRect::right(ui::px_p(3.0)),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
            parent.spawn(
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load(theme::FONT),
                        font_size: ui::FONT_SIZE * MENU_ENTRY_RATIO,
                        color: theme::FG_COLOR,
                    },
                )
                .with_style(Style {
                    margin: UiRect::right(ui::px_p(2.0)),
                    ..default()
                }),
            );
        })
        .id()
}

pub fn init_display(
    commands: &mut Commands,
    window: &Query<&Window>,
    g_trans: &GlobalTransform,
    node: &Node,
    fg_entity: Entity,
) {
    if let Some(mut entity_commands) = commands.get_entity(fg_entity) {
        entity_commands.with_children(|parent| {
            let world_pos = Vec2::new(g_trans.translation().x, g_trans.translation().y);
            let center_pos = to_canvas_pos(&window, world_pos);
            let node_half_size = Vec2::new(node.size().x, node.size().y) / 2.0;
            let mut path_builder = PathBuilder::new();
            path_builder.move_to(center_pos + Vec2::new(node_half_size.x, node_half_size.y));
            path_builder.line_to(center_pos + Vec2::new(node_half_size.x, -node_half_size.y));
            path_builder.line_to(center_pos + Vec2::new(-node_half_size.x, -node_half_size.y));
            path_builder.line_to(center_pos + Vec2::new(-node_half_size.x, node_half_size.y));
            path_builder.close();
            parent.spawn((
                ShapeBundle {
                    path: path_builder.build(),
                    ..default()
                },
                Stroke::new(theme::HIGHLIGHT_COLOR, MENU_ENTRY_B),
            ));
        });
    }
}
