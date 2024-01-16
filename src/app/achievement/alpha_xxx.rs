use crate::app::achievement::*;

pub struct AchievementDef;

const TARGET_ALPHA_COUNT: u32 = 360;

impl AchievementDefBase for AchievementDef {
    fn code(&self) -> &str {
        "alpha_xxx"
    }
    fn name(&self) -> &str {
        "Ninja"
    }
    fn color(&self) -> Color {
        Color::rgb(0.97, 0.45, 0.14)
    }
    fn description(&self) -> String {
        format!("# of alpha >= {}", TARGET_ALPHA_COUNT)
    }
    fn check_done(&self, status: &ResMut<status::ReactorStatus>) -> (u32, u32, bool) {
        let current = status.fetch("alpha_count");
        let is_done = current >= TARGET_ALPHA_COUNT;
        (current, TARGET_ALPHA_COUNT, is_done)
    }
    fn progress_ui(&self) -> AchievementProgressUi {
        AchievementProgressUi::Bar
    }
}
