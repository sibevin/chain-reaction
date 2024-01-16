use crate::{app::achievement::*, reactor};

pub struct AchievementDef;

const TARGET_SCORE: u32 = 36000;

impl AchievementDefBase for AchievementDef {
    fn code(&self) -> &str {
        "score_xxx"
    }
    fn name(&self) -> &str {
        "Archaeologist"
    }
    fn color(&self) -> Color {
        Color::rgb(0.6, 0.23, 0.06)
    }
    fn description(&self) -> String {
        format!("Score >= {}", TARGET_SCORE)
    }
    fn check_done(&self, status: &ResMut<status::ReactorStatus>) -> (u32, u32, bool) {
        let current = status.fetch("score");
        let is_done = current >= TARGET_SCORE;
        (current, TARGET_SCORE, is_done)
    }
    fn progress_ui(&self) -> AchievementProgressUi {
        AchievementProgressUi::Bar
    }
    fn format_value(&self, value: u32) -> String {
        reactor::field::format_field_text("score", value)
    }
}
