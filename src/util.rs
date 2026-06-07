use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Bounds(pub(crate) Rect);

#[derive(Component)]
pub(crate) struct LifeSpan(pub(crate) f32);

pub(crate) fn update_life_span(
    mut commands: Commands,
    mut query: Query<(Entity, &mut LifeSpan)>,
    time: Res<Time>,
) {
    for (entity, mut life_span) in &mut query {
        life_span.0 -= time.delta().as_secs_f32();
        if life_span.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
