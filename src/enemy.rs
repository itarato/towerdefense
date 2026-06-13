use crate::health::Health;
use bevy::{color::palettes::css::WHITE, prelude::*};
use std::time::Duration;

pub(crate) const ENEMY_SIZE_RADIUS: f32 = 12.0;

pub(crate) const ENEMY_SPECS: [EnemySpecs; 3] = [
    EnemySpecs {
        speed: 120.0,
        health: 40,
        color: Color::Srgba(Srgba::new(0.6, 0.4, 0.2, 1.0)),
    },
    EnemySpecs {
        speed: 80.0,
        health: 100,
        color: Color::Srgba(Srgba::new(0.8, 0.6, 0.4, 1.0)),
    },
    EnemySpecs {
        speed: 50.0,
        health: 200,
        color: Color::Srgba(Srgba::new(1.0, 0.8, 0.6, 1.0)),
    },
];

pub(crate) struct EnemySpecs {
    pub(crate) speed: f32,
    health: i32,
    color: Color,
}

pub(crate) struct Burst {
    pub(crate) kind: u8,
    pub(crate) count: u8,

    start_timer: Timer,
    spawn_timer: Timer,
}

impl Burst {
    fn update(&mut self, frame_delta: Duration) -> Vec<u8> {
        self.start_timer.tick(frame_delta);
        if !self.start_timer.is_finished() {
            return vec![];
        }

        if self.spawn_timer.tick(frame_delta).just_finished() {
            self.count -= 1;
            return vec![self.kind];
        }

        vec![]
    }

    fn is_completed(&self) -> bool {
        self.count == 0
    }
}

pub(crate) struct Wave {
    pub(crate) bursts: Vec<Burst>,
}

impl Wave {
    fn update(&mut self, frame_delta: Duration) -> Vec<u8> {
        let mut spawn_kinds = vec![];
        for burst in &mut self.bursts {
            spawn_kinds.append(&mut burst.update(frame_delta));
        }

        self.bursts.retain(|burst| !burst.is_completed());

        spawn_kinds
    }

    fn is_completed(&self) -> bool {
        self.bursts.is_empty()
    }
}

enum WavesState {
    WaitingForNextWave,
    WaveInProgress,
    Completed,
}

pub(crate) struct Waves {
    pub(crate) waves: Vec<Wave>,
    state: WavesState,
}

impl Waves {
    pub(crate) fn load() -> Self {
        Self {
            waves: vec![
                Wave {
                    bursts: vec![
                        Burst {
                            count: 20,
                            kind: 0,
                            start_timer: Timer::from_seconds(2.0, TimerMode::Once),
                            spawn_timer: Timer::from_seconds(0.3, TimerMode::Repeating),
                        },
                        Burst {
                            count: 10,
                            kind: 1,
                            start_timer: Timer::from_seconds(2.5, TimerMode::Once),
                            spawn_timer: Timer::from_seconds(0.6, TimerMode::Repeating),
                        },
                    ],
                },
                Wave {
                    bursts: vec![Burst {
                        count: 10,
                        kind: 2,
                        start_timer: Timer::from_seconds(1.0, TimerMode::Once),
                        spawn_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                    }],
                },
            ],
            state: WavesState::WaitingForNextWave,
        }
    }

    pub(crate) fn update(&mut self, frame_delta: Duration) -> Vec<u8> {
        match self.state {
            WavesState::WaitingForNextWave => {
                if self.waves.is_empty() {
                    return vec![];
                }

                println!("Next waves starts");
                self.state = WavesState::WaveInProgress;
            }
            WavesState::WaveInProgress => {
                let spawn_kinds = self.waves.first_mut().unwrap().update(frame_delta);

                if self.waves.first().unwrap().is_completed() {
                    self.waves.remove(0);

                    if self.waves.is_empty() {
                        println!("Waves are completed");
                        self.state = WavesState::Completed;
                    } else {
                        self.state = WavesState::WaitingForNextWave;
                    }
                }

                return spawn_kinds;
            }
            WavesState::Completed => {}
        }

        vec![]
    }
}

pub(crate) fn spawn_enemy(
    kind: u8,
    commands: &mut Commands,
    pos: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let shape = meshes.add(RegularPolygon::new(ENEMY_SIZE_RADIUS, 7));
    commands
        .spawn((
            Enemy(kind),
            Health::new(ENEMY_SPECS[kind as usize].health),
            Mesh2d(shape),
            MeshMaterial2d(materials.add(ENEMY_SPECS[kind as usize].color)),
            Transform::from_xyz(pos.x, pos.y, 0.0),
        ))
        .with_child((
            Text2d::new("100%"),
            TextFont {
                font_size: 12.0,
                ..default()
            },
            TextColor(WHITE.into()),
            Transform::from_xyz(0.0, -16.0, 0.0),
            EnemyHealthText,
        ));
}

#[derive(Component)]
pub(crate) struct Enemy(pub(crate) u8);

#[derive(Component)]
pub(crate) struct EnemyHealthText;
