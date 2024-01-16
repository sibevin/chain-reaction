use crate::{app::achievement::*, reactor::particle};

pub struct AchievementDef;

impl AchievementDefBase for AchievementDef {
    fn code(&self) -> &str {
        "max_h"
    }
    fn name(&self) -> &str {
        "Slime"
    }
    fn color(&self) -> Color {
        Color::rgb(0.22, 0.60, 0.97)
    }
    fn description(&self) -> String {
        String::from("Get a max-level H")
    }
    fn check_done(&self, status: &ResMut<status::ReactorStatus>) -> (u32, u32, bool) {
        let current = status.fetch("current_max_hyper_level");
        let total = particle::hyper::MAX_LEVEL as u32;
        let is_done = current >= total;
        (current, total, is_done)
    }
    fn progress_ui(&self) -> AchievementProgressUi {
        AchievementProgressUi::Dots
    }
}
