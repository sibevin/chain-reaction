use crate::app::achievement::*;

pub struct AchievementDef;

const TARGET_ACH_COUNT: u32 = 5;

impl AchievementDefBase for AchievementDef {
    fn code(&self) -> &str {
        "achievement_xxx"
    }
    fn name(&self) -> &str {
        "Ace"
    }
    fn color(&self) -> Color {
        Color::rgb(0.55, 0.45, 0.96)
    }
    fn description(&self) -> String {
        format!("Get {} marks in a game", TARGET_ACH_COUNT)
    }
    fn check_done(&self, status: &ResMut<status::AppStatus>) -> (u32, u32, bool) {
        let current = status.done_achievements.len() as u32;
        let is_done = current >= TARGET_ACH_COUNT;
        (current, TARGET_ACH_COUNT, is_done)
    }
    fn progress_ui(&self) -> AchievementProgressUi {
        AchievementProgressUi::Dots
    }
}
