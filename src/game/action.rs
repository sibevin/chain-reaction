use super::*;
use rand::{thread_rng, Rng};

pub fn get_field_rect(padding: f32) -> Rect {
    Rect::new(
        -FIELD_W / 2.0 + padding,
        -FIELD_H / 2.0 + FIELD_BAR_H + padding,
        FIELD_W / 2.0 - padding,
        FIELD_H / 2.0 - FIELD_BAR_H - padding,
    )
}

pub fn gen_random_pos_in_field(padding: f32) -> Vec2 {
    let mut rng = thread_rng();
    let rect = get_field_rect(padding);
    Vec2::new(
        rng.gen_range(rect.min.x..rect.max.x),
        rng.gen_range(rect.min.y..rect.max.y),
    )
}

pub fn format_field_text(field: &str, value: u32) -> String {
    if field == "time" {
        format!("{:0>4}.{:0>2}", value / 100, value % 100)
    } else if field == "alpha_count" {
        format!("{:0>4}", value)
    } else if field == "score" {
        let value_str = format!("{:0>6}", value);
        let (first, second) = value_str.split_at(3);
        format!("{},{}", first, second)
    } else if field == "chain_length" {
        format!("{:0>4}", value)
    } else {
        format!("{}", value)
    }
}
