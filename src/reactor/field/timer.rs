use crate::reactor::{field::*, ReactorTimer};

pub fn reset_field(mut field_timer_query: Query<(&mut Text, &mut FieldTimer), With<FieldTimer>>) {
    for (mut text, mut field_timer) in field_timer_query.iter_mut() {
        field_timer.0 = 0;
        text.sections[0].value = format_field_text("time", field_timer.0);
    }
}

const SCORE_PER_SECOND: u32 = 10;

pub fn update_field(
    time: Res<Time>,
    mut timer_query: Query<&mut ReactorTimer>,
    mut field_timer_query: Query<
        (&mut Text, &mut FieldTimer),
        (With<FieldTimer>, Without<FieldScore>),
    >,
    mut field_score_query: Query<
        (&mut Text, &mut FieldScore),
        (With<FieldScore>, Without<FieldTimer>),
    >,
) {
    for mut timer in &mut timer_query {
        if timer.tick(time.delta()).just_finished() {
            let (mut text, mut field_timer) = field_timer_query.single_mut();
            field_timer.0 += 1;
            text.sections[0].value = format_field_text("time", field_timer.0);
            let (mut text, mut field_score) = field_score_query.single_mut();
            if field_timer.0 > 0 && field_timer.0 % 100 == 0 {
                field_score.0 += SCORE_PER_SECOND;
                text.sections[0].value = format_field_text("score", field_score.0);
            }
        }
    }
}
