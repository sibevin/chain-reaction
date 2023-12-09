use crate::reactor::{field::*, ReactorTimer};

pub fn reset_field(mut field_timer_query: Query<(&mut Text, &mut FieldTimer), With<FieldTimer>>) {
    for (mut text, mut field_timer) in field_timer_query.iter_mut() {
        field_timer.0 = 0;
        text.sections[0].value = format_field_text("time", field_timer.0);
    }
}

pub fn update_field(
    time: Res<Time>,
    mut timer_query: Query<&mut ReactorTimer>,
    mut field_timer_query: Query<(&mut Text, &mut FieldTimer), With<FieldTimer>>,
) {
    for mut timer in &mut timer_query {
        if timer.tick(time.delta()).just_finished() {
            for (mut text, mut field_timer) in field_timer_query.iter_mut() {
                field_timer.0 = field_timer.0 + 1;
                text.sections[0].value = format_field_text("time", field_timer.0);
            }
        }
    }
}
