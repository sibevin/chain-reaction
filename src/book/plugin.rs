use crate::book::*;

pub struct BookPlugin;

impl Plugin for BookPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PageState>();
        for page in PAGES {
            page.build(app);
        }
    }
}
