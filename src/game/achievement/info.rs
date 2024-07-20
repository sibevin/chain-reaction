use super::*;

const MAX_RUNNING_ACH_COUNT: usize = 2;

#[derive(Resource, Default)]
pub struct AchievementInfo {
    done_codes: Vec<String>,
    running_codes: Vec<String>,
    next_done_code: Option<String>,
}

impl AchievementInfo {
    pub fn reset(&mut self, store: &AchievementStore) {
        self.update_running_codes(store);
        self.done_codes = vec![];
    }

    pub fn update_running_codes(&mut self, store: &AchievementStore) {
        self.running_codes = vec![];
        for code in store.pinned_codes.iter() {
            if store.is_done(code) {
                continue;
            }
            self.running_codes.push(String::from(code));
            if self.running_codes.len() >= MAX_RUNNING_ACH_COUNT {
                return;
            }
        }
        for ach_def in ACHIEVEMENTS {
            if store.is_done(ach_def.code()) || store.is_pinned(ach_def.code()) {
                continue;
            }
            self.running_codes.push(String::from(ach_def.code()));
            if self.running_codes.len() >= MAX_RUNNING_ACH_COUNT {
                return;
            }
        }
    }

    pub fn is_running(&self, code: &str) -> bool {
        self.running_codes.contains(&String::from(code))
    }

    pub fn running_codes(&self) -> Vec<String> {
        self.running_codes.clone()
    }

    pub fn push_to_done(&mut self, code: &str) {
        self.done_codes.insert(0, String::from(code));
    }

    pub fn fetch_next_done(&mut self) {
        self.next_done_code = self.done_codes.pop();
    }

    pub fn next_done_code(&self) -> Option<String> {
        if let Some(code) = &self.next_done_code {
            Some(code.clone())
        } else {
            None
        }
    }
}
