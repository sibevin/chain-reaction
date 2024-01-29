use crate::reactor::anime_effect::*;

#[derive(Default)]
pub struct AnimeEffectLens {
    start_radius: f32,
    start_color_alpha: f32,
    start_border: f32,
    start_pos: Vec2,
    end_radius: f32,
    end_color_alpha: f32,
    end_border: f32,
    end_pos: Vec2,
}

impl AnimeEffectLens {
    pub fn new(
        start_radius: f32,
        start_color_alpha: f32,
        start_border: f32,
        start_pos: Vec2,
        end_radius: f32,
        end_color_alpha: f32,
        end_border: f32,
        end_pos: Vec2,
    ) -> Self {
        Self {
            start_radius,
            start_color_alpha,
            start_border,
            start_pos,
            end_radius,
            end_color_alpha,
            end_border,
            end_pos,
        }
    }
}

impl Lens<AnimeEffect> for AnimeEffectLens {
    fn lerp(&mut self, target: &mut AnimeEffect, ratio: f32) {
        target.radius = self.start_radius + (self.end_radius - self.start_radius) * ratio;
        target.current_pos.x = self.start_pos.x + (self.end_pos.x - self.start_pos.x) * ratio;
        target.current_pos.y = self.start_pos.y + (self.end_pos.y - self.start_pos.y) * ratio;
        let color_alpha =
            self.start_color_alpha + (self.end_color_alpha - self.start_color_alpha) * ratio;
        target.color.set_a(color_alpha);
        target.border = self.start_border + (self.end_border - self.start_border) * ratio;
        target.rotation += target.rotation_delta;
    }
}
