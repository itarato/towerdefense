use bevy::prelude::*;

pub(crate) const ENEMY_SIZE_RADIUS: f32 = 8.0;

pub(crate) struct Burst {
    pub(crate) kind: u8,
    pub(crate) count: u8,
    pub(crate) delay: f32,
    pub(crate) frequency: f32,
}

pub(crate) struct Wave {
    pub(crate) bursts: Vec<Burst>,
}

pub(crate) struct Waves(pub(crate) Vec<Wave>);

#[derive(Component)]
pub(crate) struct Enemy;

#[derive(Resource)]
pub(crate) struct EnemySpawnTimer(pub(crate) Timer);

#[derive(Component)]
pub(crate) struct EnemyHealthText;
