use crate::{app::achievement::*, reactor};

pub struct AchievementDef;

const TARGET_TIME_SECS: u32 = 120;

impl AchievementDefBase for AchievementDef {
    fn code(&self) -> &str {
        "time_xxx_s"
    }
    fn name(&self) -> &str {
        "Survivor"
    }
    fn color(&self) -> Color {
        Color::rgb(0.6, 0.6, 0.6)
    }
    fn description(&self) -> String {
        format!("Time >= {}s", TARGET_TIME_SECS)
    }
    fn check_done(&self, status: &ResMut<status::ReactorStatus>) -> (u32, u32, bool) {
        let current = status.fetch("time");
        let is_done = current >= TARGET_TIME_SECS * 100;
        (current, TARGET_TIME_SECS * 100, is_done)
    }
    fn progress_ui(&self) -> AchievementProgressUi {
        AchievementProgressUi::Bar
    }
    fn format_value(&self, value: u32) -> String {
        reactor::field::format_field_text("time", value)
    }
}
