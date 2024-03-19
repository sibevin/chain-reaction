use super::*;

#[derive(Resource)]
pub struct ElementStatus {
    pub is_refresh_required: bool,
}

impl Default for ElementStatus {
    fn default() -> Self {
        ElementStatus {
            is_refresh_required: true,
        }
    }
}
