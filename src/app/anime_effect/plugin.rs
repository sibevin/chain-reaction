use crate::app::anime_effect::*;
use std::collections::HashSet;

pub struct AnimeEffectPlugin;

#[derive(Resource)]
struct AnimeEffectTimer(pub Timer);

const ANIME_FRAME_SECS: f32 = 0.02;

impl Plugin for AnimeEffectPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimeEffectStatus::default())
            .insert_resource(AnimeEffectTimer(Timer::from_seconds(
                ANIME_FRAME_SECS,
                TimerMode::Repeating,
            )))
            .add_systems(
                Update,
                (
                    handle_anime_effect,
                    component_animator_system::<AnimeEffect>,
                ),
            );
    }
}

fn handle_anime_effect(
    mut commands: Commands,
    mut ae_query: Query<(Entity, &mut AnimeEffect), With<AnimeEffect>>,
    mut tween_completed_events: EventReader<TweenCompleted>,
    mut timer: ResMut<AnimeEffectTimer>,
    mut ae_status: ResMut<AnimeEffectStatus>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (_, mut ae) in ae_query.iter_mut() {
            update_anime_effect(&mut commands, &mut ae);
        }
    }
    for tween_event in tween_completed_events.read() {
        if tween_event.user_data == ANIME_EFFECT_DONE_EVENT {
            ae_status.entities_to_despawn.insert(tween_event.entity);
        }
    }
    let mut despawned_entities: HashSet<Entity> = HashSet::new();
    for entity in ae_status.entities_to_despawn.iter() {
        if let Some(entity_commands) = commands.get_entity(*entity) {
            entity_commands.despawn_recursive();
            despawned_entities.insert(*entity);
        }
    }
    for entity in despawned_entities.iter() {
        ae_status.entities_to_despawn.remove(entity);
    }
}
