use crate::app::anime_effect::*;
use std::collections::HashSet;

#[derive(Resource, Default, Debug)]
pub struct AnimeEffectStatus {
    pub entities_to_despawn: HashSet<Entity>,
}

pub fn despawn_anime_effect(entity: Entity, ae_status: &mut ResMut<AnimeEffectStatus>) {
    ae_status.entities_to_despawn.insert(entity);
}
