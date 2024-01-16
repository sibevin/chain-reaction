use crate::app::achievement::*;

pub struct AchievementDef;

const TARGET_CHAIN_LENGTH: u32 = 48;

impl AchievementDefBase for AchievementDef {
    fn code(&self) -> &str {
        "c_chain_xxx"
    }
    fn name(&self) -> &str {
        "Fairy"
    }
    fn color(&self) -> Color {
        Color::rgb(0.20, 0.82, 0.39)
    }
    fn description(&self) -> String {
        format!("C-Chain length >= {}", TARGET_CHAIN_LENGTH)
    }
    fn check_done(&self, status: &ResMut<status::ReactorStatus>) -> (u32, u32, bool) {
        let current = match status.current_chain() {
            status::StatusChain::Control => status.fetch("chain_length"),
            _ => 0,
        };
        let is_done = current >= TARGET_CHAIN_LENGTH;
        (current, TARGET_CHAIN_LENGTH, is_done)
    }
    fn progress_ui(&self) -> AchievementProgressUi {
        AchievementProgressUi::Bar
    }
}
