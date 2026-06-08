use bevy::prelude::*;

pub(crate) const ENEMY_SIZE_RADIUS: f32 = 8.0;

#[derive(Component)]
pub(crate) struct Enemy;

#[derive(Resource)]
pub(crate) struct EnemySpawnTimer(pub(crate) Timer);

#[derive(Component)]
pub(crate) struct EnemyHealthText;
