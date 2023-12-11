use crate::reactor::{field::*, particle::*, ReactorTimer};

pub fn reset_field(
    mut field_ac_query: Query<(&mut Text, &mut FieldAlphaCount), With<FieldAlphaCount>>,
) {
    for (mut text, mut field_alpha_count) in field_ac_query.iter_mut() {
        field_alpha_count.0 = 0;
        text.sections[0].value = format_field_text("alpha_count", field_alpha_count.0);
    }
}

pub fn update_field(
    time: Res<Time>,
    mut timer_query: Query<&mut ReactorTimer>,
    particle_query: Query<&Particle, With<Particle>>,
    mut field_alpha_count_query: Query<(&mut Text, &mut FieldAlphaCount), With<FieldAlphaCount>>,
) {
    for mut timer in &mut timer_query {
        if timer.tick(time.delta()).just_finished() {
            let mut total_alpha_count = 0;
            for particle in particle_query.iter() {
                if particle.particle_type() == ParticleType::Alpha {
                    total_alpha_count += 1;
                }
            }
            for (mut text, mut field_alpha_count) in field_alpha_count_query.iter_mut() {
                field_alpha_count.0 = total_alpha_count;
                text.sections[0].value = format_field_text("alpha_count", total_alpha_count);
            }
        }
    }
}
