use crate::app::achievement::*;

pub struct AchievementDef;

const TARGET_C_COUNT: u32 = 10;

impl AchievementDefBase for AchievementDef {
    fn code(&self) -> &str {
        "max_c_xxx"
    }
    fn name(&self) -> &str {
        "Gobline King"
    }
    fn color(&self) -> Color {
        Color::rgb(0.56, 0.64, 0.16)
    }
    fn description(&self) -> String {
        format!("# of max-level C >= {}", TARGET_C_COUNT)
    }
    fn check_done(&self, status: &ResMut<status::ReactorStatus>) -> (u32, u32, bool) {
        let current = status.fetch("current_full_level_control_count");
        let is_done = current >= TARGET_C_COUNT;
        (current, TARGET_C_COUNT, is_done)
    }
    fn progress_ui(&self) -> AchievementProgressUi {
        AchievementProgressUi::Dots
    }
}
