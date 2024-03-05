use crate::app::anime_effect::*;

#[derive(Default)]
pub struct AnimeEffectLens {
    width: (f32, f32),
}

impl AnimeEffectLens {
    pub fn new(width: (f32, f32)) -> Self {
        Self { width }
    }
}

impl Lens<AnimeEffect> for AnimeEffectLens {
    fn lerp(&mut self, target: &mut AnimeEffect, ratio: f32) {
        target.delta = ratio;
        target.width = self.width.0 + (self.width.1 - self.width.0) * ratio;
    }
}
