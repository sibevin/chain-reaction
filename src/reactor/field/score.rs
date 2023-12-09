use crate::reactor::field::*;

#[derive(Component, Deref, DerefMut)]
pub struct ScoreTimer(pub Timer);

pub fn init_field(mut commands: Commands) {
    commands.spawn(ScoreTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
}

pub fn reset_field(mut field_score_query: Query<(&mut Text, &mut FieldScore), With<FieldScore>>) {
    for (mut text, mut field_score) in field_score_query.iter_mut() {
        field_score.0 = 0;
        text.sections[0].value = format_field_text("score", field_score.0);
    }
}

pub fn update_field(
    time: Res<Time>,
    mut score_timer_query: Query<&mut ScoreTimer>,
    mut field_score_query: Query<(&mut Text, &mut FieldScore), With<FieldScore>>,
    field_alpha_count_query: Query<&FieldAlphaCount, With<FieldAlphaCount>>,
) {
    for mut timer in &mut score_timer_query {
        if timer.tick(time.delta()).just_finished() {
            for (mut text, mut field_score) in field_score_query.iter_mut() {
                let alpha_count = field_alpha_count_query.single().0;
                field_score.0 = field_score.0 + alpha_count;
                text.sections[0].value = format_field_text("score", field_score.0);
            }
        }
    }
}
