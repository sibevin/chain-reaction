use super::*;

pub mod achievement_xxx;
pub mod alpha_xxx;
pub mod c_chain_xxx;
pub mod h_chain_xxx;
pub mod max_c_xxx;
pub mod max_h;
pub mod not_moving_xxx_s;
pub mod score_xxx;
pub mod time_xxx_s;

pub const ACHIEVEMENTS: [&dyn AchievementKindBase; 9] = [
    &max_h::AchievementDef,
    &max_c_xxx::AchievementDef,
    &h_chain_xxx::AchievementDef,
    &c_chain_xxx::AchievementDef,
    &alpha_xxx::AchievementDef,
    &time_xxx_s::AchievementDef,
    &not_moving_xxx_s::AchievementDef,
    &score_xxx::AchievementDef,
    &achievement_xxx::AchievementDef,
];

#[derive(Clone, PartialEq, Debug)]
pub enum AchievementUiStatus {
    Normal,
    Pinned,
    Done,
}

#[derive(Clone, PartialEq, Debug)]
pub enum AchievementProgressUi {
    Bar,
    Dots,
}
pub trait AchievementKindBase {
    fn code(&self) -> &str;
    fn name(&self) -> &str;
    fn color(&self) -> Color;
    fn icon(&self) -> &str {
        self.code()
    }
    fn description(&self) -> String;
    fn check_done(&self, status: &ResMut<GameStatus>) -> (u32, u32, bool);
    fn build_empty_record(&self) -> AchievementRecord {
        AchievementRecord {
            code: String::from(self.code()),
            is_done: false,
            done_at: String::from(""),
        }
    }
    fn icon_path(&self) -> String {
        format!("images/achievement/{}.png", self.icon())
    }
    fn progress_ui(&self) -> AchievementProgressUi;
    fn format_value(&self, value: u32) -> String {
        format!("{}", value)
    }
}

pub fn fetch_ach_def(code: &str) -> &dyn AchievementKindBase {
    for ach_def in ACHIEVEMENTS {
        if ach_def.code() == code {
            return ach_def;
        }
    }
    panic!("Invalid achievement code")
}
