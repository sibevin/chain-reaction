use super::*;
use bevy::prelude::*;

pub struct StudioPlugin;

impl Plugin for StudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<StudioState>()
            .insert_resource(StudioStatus::default());
        for state in STUDIO_STATES {
            state.build(app);
        }
    }
}
