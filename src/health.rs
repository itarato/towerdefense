use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Health {
    max: u32,
    pub(crate) current: u32,
}

impl Health {
    pub(crate) fn new(max: u32) -> Self {
        Self { max, current: max }
    }

    pub(crate) fn percentage(&self) -> f32 {
        self.current as f32 / self.max as f32
    }

    pub(crate) fn is_dead(&self) -> bool {
        self.current <= 0
    }
}

pub(crate) fn update_health(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in &query {
        if health.is_dead() {
            commands.entity(entity).despawn();
        }
    }
}
