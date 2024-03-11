use super::*;

pub struct AnimeEffectKindLineShoot;

impl AnimeEffectKindBase for AnimeEffectKindLineShoot {
    fn create(&self, commands: &mut Commands, param: AnimeEffectParam) -> Entity {
        let root_entity = commands
            .spawn((SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, ANIME_EFFECT_Z_INDEX),
                ..default()
            },))
            .id();
        let ae = AnimeEffect {
            kind: param.kind,
            segments: vec![],
            pos_1: param.pos_1,
            pos_2: param.pos_2,
            layer: 0,
            delta: 0.0,
            color: param.color,
            width: param.width_start,
            radius: 0.0,
            root_entity,
            is_done: false,
        };
        let tween = Tween::new(
            EaseFunction::CubicOut,
            Duration::from_millis(500),
            lens::AnimeEffectLens::new((param.width_start, param.width_end)),
        );
        commands
            .entity(root_entity)
            .insert((ae, Animator::new(tween)));
        root_entity
    }

    fn draw(&self, commands: &mut Commands, ae: &mut AnimeEffect) {
        if ae.is_done {
            return;
        }
        if let Some(mut entity_commands) = commands.get_entity(ae.root_entity) {
            entity_commands.despawn_descendants();
            entity_commands.with_children(|parent| {
                parent
                    .spawn((SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 0.0001),
                        ..default()
                    },))
                    .with_children(|parent| {
                        let mut path_builder = PathBuilder::new();
                        path_builder.move_to(ae.pos_1);
                        let ratio_end_pos = ae.pos_1 + (ae.pos_2 - ae.pos_1) * ae.delta;
                        path_builder.line_to(ratio_end_pos);
                        parent.spawn((
                            ShapeBundle {
                                path: path_builder.build(),
                                ..default()
                            },
                            Stroke::new(ae.color, ae.width),
                        ));
                        let circle = shapes::Circle {
                            radius: ae.width * 1.5,
                            center: ratio_end_pos,
                        };
                        let geo_builder = GeometryBuilder::new().add(&circle);
                        parent.spawn((
                            ShapeBundle {
                                path: geo_builder.build(),
                                ..default()
                            },
                            Fill::color(ae.color),
                        ));
                    });
            });
            if ae.delta == 1.0 {
                ae.is_done = true;
            }
        };
    }
}
