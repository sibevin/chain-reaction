use crate::{app::achievement::*, reactor};

pub struct AchievementDef;

const TARGET_NOT_MOVING_SECS: u32 = 60;

impl AchievementDefBase for AchievementDef {
    fn code(&self) -> &str {
        "not_moving_xxx_s"
    }
    fn name(&self) -> &str {
        "Fortress"
    }
    fn color(&self) -> Color {
        Color::rgb(1.0, 0.0, 0.0)
    }
    fn description(&self) -> String {
        format!("Not moving {}s", TARGET_NOT_MOVING_SECS)
    }
    fn check_done(&self, status: &ResMut<status::ReactorStatus>) -> (u32, u32, bool) {
        let current = status.fetch("total_stopping_time");
        let is_done = current >= TARGET_NOT_MOVING_SECS * 100;
        (current, TARGET_NOT_MOVING_SECS * 100, is_done)
    }
    fn progress_ui(&self) -> AchievementProgressUi {
        AchievementProgressUi::Bar
    }
    fn format_value(&self, value: u32) -> String {
        reactor::field::format_field_text("time", value)
    }
}
