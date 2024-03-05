use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct AppCursorData {
    pub window_pos: Vec2,
    pub canvas_pos: Vec2,
}

pub struct AppCursorPlugin;

impl Plugin for AppCursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AppCursorData::default())
            .add_systems(Update, update_cursor_pos);
    }
}

fn update_cursor_pos(
    mut cursor_data: ResMut<AppCursorData>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    window: Query<&Window>,
) {
    for cursor_event in cursor_moved_events.read() {
        cursor_data.window_pos = cursor_event.position;
        let window = window.single();
        let win_w = window.resolution.width();
        let win_h = window.resolution.height();
        cursor_data.canvas_pos = Vec2::new(cursor_event.position.x, -cursor_event.position.y)
            - Vec2::new(win_w / 2.0, -win_h / 2.0);
    }
}
