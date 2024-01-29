use crate::reactor::anime_effect::*;

#[derive(Default)]
pub struct AnimeEffectLens {
    radius: (f32, f32),
    color_alpha: (f32, f32),
    border: (f32, f32),
    position: (Vec2, Vec2),
}

impl AnimeEffectLens {
    pub fn new(
        radius: (f32, f32),
        color_alpha: (f32, f32),
        border: (f32, f32),
        position: (Vec2, Vec2),
    ) -> Self {
        Self {
            radius,
            color_alpha,
            border,
            position,
        }
    }
}

impl Lens<AnimeEffect> for AnimeEffectLens {
    fn lerp(&mut self, target: &mut AnimeEffect, ratio: f32) {
        target.radius = self.radius.0 + (self.radius.1 - self.radius.0) * ratio;
        target.current_pos.x = self.position.0.x + (self.position.1.x - self.position.0.x) * ratio;
        target.current_pos.y = self.position.0.y + (self.position.1.y - self.position.0.y) * ratio;
        let color_alpha = self.color_alpha.0 + (self.color_alpha.1 - self.color_alpha.0) * ratio;
        target.color.set_a(color_alpha);
        target.border = self.border.0 + (self.border.1 - self.border.0) * ratio;
        target.rotation += target.rotation_delta;
    }
}
