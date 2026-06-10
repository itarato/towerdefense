use std::time::Duration;

use bevy::prelude::*;

pub(crate) const ENEMY_SIZE_RADIUS: f32 = 8.0;

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
                            spawn_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                        },
                    ],
                },
                Wave {
                    bursts: vec![Burst {
                        count: 10,
                        kind: 2,
                        start_timer: Timer::from_seconds(1.0, TimerMode::Once),
                        spawn_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
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

#[derive(Component)]
pub(crate) struct Enemy;

#[derive(Resource)]
pub(crate) struct EnemySpawnTimer(pub(crate) Timer);

#[derive(Component)]
pub(crate) struct EnemyHealthText;
