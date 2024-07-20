use super::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(status::GameStatus::default())
            .insert_resource(timer::GameRefreshTimer(Timer::from_seconds(
                timer::GAME_REFRESH_FRAME_SECS,
                TimerMode::Repeating,
            )))
            .insert_resource(timer::GameBuildTimer(Timer::from_seconds(
                timer::GAME_BUILD_DELAY_SECS,
                TimerMode::Once,
            )))
            .insert_resource(timer::GameThrottleTimer(Timer::from_seconds(
                timer::GAME_THROTTLE_SECS,
                TimerMode::Repeating,
            )))
            .insert_resource(timer::GameScoreboardTimer(Timer::from_seconds(
                timer::GAME_SCOREBOARD_SECS,
                TimerMode::Repeating,
            )))
            .add_state::<PhaseState>()
            .add_systems(
                Update,
                (anime_end::handle_anime_end_events, field::field_systems()),
            )
            .add_plugins((
                achievement::AchievementPlugin,
                leaderboard::LeaderboardPlugin,
            ));
        for phase in phase::PHASES {
            phase.build(app);
        }
    }
}
