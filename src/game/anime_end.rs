use super::achievement;
use bevy::prelude::*;
use bevy_tweening::TweenCompleted;

pub const ACH_DONE_ANIME_END: u64 = 1;

pub fn handle_anime_end_events(
    mut tween_completed_events: EventReader<TweenCompleted>,
    mut ach_info: ResMut<achievement::AchievementInfo>,
) {
    for tween_event in tween_completed_events.read() {
        match tween_event.user_data {
            ACH_DONE_ANIME_END => {
                ach_info.fetch_next_done();
            }
            _ => (),
        }
    }
}
