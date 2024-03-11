use super::*;

#[derive(Resource, Default)]
pub struct ElementStatus {
    pub fg_entity: Option<Entity>,
    pub bg_entity: Option<Entity>,
}
