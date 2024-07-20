use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AchievementRecord {
    pub code: String,
    pub is_done: bool,
    pub done_at: String,
}
