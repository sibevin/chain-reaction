use crate::app::{self, cursor::*, layer, theme};
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_ui_navigation::NavRequestSystem;
use phf::phf_map;

pub struct AppCursorIconPlugin;

#[derive(Resource)]
struct AppCursorIconThrottleTimer(pub Timer);

impl Plugin for AppCursorIconPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AppCursorIconThrottleTimer(Timer::from_seconds(
            THROTTLE_SECS,
            TimerMode::Repeating,
        )))
        .add_systems(Update, show_cursor_icon.after(NavRequestSystem));
    }
}

#[derive(Component)]
pub struct AppCursorIcon {
    pub kind: String,
    pub is_loaded: bool,
}

struct CursorData {
    pub origin: Vec2,
    pub color: Color,
    pub path: &'static str,
}

static CURSOR_DATA_MAP: phf::Map<&'static str, CursorData> = phf_map! {
    "default" => CursorData {
        origin: Vec2::ZERO,
        color: theme::U_COLOR,
        path: "images/cursor/default.png",
    },
    "pointer" => CursorData {
        origin: Vec2::new(-CURSOR_ICON_SIZE * 0.5, 0.0),
        color: theme::U_COLOR,
        path: "images/cursor/pointer.png",
    },
    "move" => CursorData {
        origin: Vec2::new(-CURSOR_ICON_SIZE * 0.5, -CURSOR_ICON_SIZE * 0.5),
        color: theme::U_COLOR,
        path: "images/cursor/move.png",
    },
};

const CURSOR_ICON_SIZE: f32 = 36.0;

pub fn init_cursor_icon(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let cursor_data = CURSOR_DATA_MAP.get("default").unwrap();
    commands.spawn((
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Px(CURSOR_ICON_SIZE),
                height: Val::Px(CURSOR_ICON_SIZE),
                ..default()
            },
            image: UiImage::new(asset_server.load(cursor_data.path)),
            z_index: ZIndex::Global(layer::CURSOR_UI_Z_INDEX),
            ..default()
        },
        AppCursorIcon {
            kind: String::from("default"),
            is_loaded: false,
        },
        Interaction::None,
        Pickable::IGNORE,
    ));
}

pub fn set_cursor_icon(
    cursor_icon_query: &mut Query<(&mut UiImage, &mut AppCursorIcon)>,
    asset_server: &Res<AssetServer>,
    kind: &str,
) {
    if let Ok((mut image, mut cursor_icon)) = cursor_icon_query.get_single_mut() {
        if cursor_icon.kind == kind {
            return;
        }
        cursor_icon.kind = String::from(kind);
        if kind == "hidden" {
            return;
        }
        if let Some(cursor_data) = CURSOR_DATA_MAP.get(&cursor_icon.kind) {
            cursor_icon.is_loaded = false;
            asset_server.reload(cursor_data.path);
            image.texture = asset_server.load(cursor_data.path);
        }
    }
}

const HIDDEN_POS: Vec2 = Vec2::new(app::WINDOW_W * -3.0, app::WINDOW_H * -3.0);
const THROTTLE_SECS: f32 = 0.05;

fn show_cursor_icon(
    mut cursor_icon_query: Query<(&mut Style, &mut AppCursorIcon)>,
    cursor: Res<AppCursorData>,
    mut throttle_timer: ResMut<AppCursorIconThrottleTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Image>>,
) {
    if throttle_timer.0.tick(time.delta()).just_finished() {
        if let Ok((mut style, mut cursor_icon)) = cursor_icon_query.get_single_mut() {
            if let Some(cursor_data) = CURSOR_DATA_MAP.get(&cursor_icon.kind) {
                let cursor_image = asset_server.load(cursor_data.path);
                if let Some(texture) = textures.get_mut(cursor_image) {
                    if !cursor_icon.is_loaded {
                        let color_rgba = cursor_data.color.as_rgba_u8();
                        for (i, pixel) in texture.data.iter_mut().enumerate() {
                            let rgba_index = i % 4;
                            if rgba_index != 3 {
                                *pixel =
                                    (color_rgba[rgba_index] as f32 * (*pixel as f32 / 255.0)) as u8;
                            }
                        }
                    }
                    cursor_icon.is_loaded = true;
                }
                style.left = Val::Px(cursor.window_pos.x + cursor_data.origin.x);
                style.top = Val::Px(cursor.window_pos.y + cursor_data.origin.y);
            } else if cursor_icon.kind == "hidden" {
                style.left = Val::Px(cursor.window_pos.x + HIDDEN_POS.x);
                style.top = Val::Px(cursor.window_pos.y + HIDDEN_POS.y);
            }
        }
    }
}
