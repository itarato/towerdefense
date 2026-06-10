use crate::{
    deletable::update_deletables, dragged::*, enemy::*, health::*, math::*, tower::*, util::*,
};
use bevy::{
    color::palettes::css::{RED, WHITE},
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::{ButtonState, mouse::MouseButtonInput},
    prelude::*,
};
use rand::prelude::*;

#[derive(Resource)]
struct GameState {
    base_life: i32,
    waves: Waves,
}

#[derive(Resource)]
struct GameMap {
    path: Vec<Vec2>,
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct FpsText;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Text::default(),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                ..default()
            },
        ))
        .with_child((
            ScoreText,
            TextSpan::default(),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(RED.into()),
        ));

    commands
        .spawn((
            Text::new("FPS: "),
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(140.0),
                top: Val::Px(10.0),
                ..default()
            },
        ))
        .with_child((
            FpsText,
            TextSpan::default(),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(WHITE.into()),
        ));

    // Spawnable towers:
    let tower_kind_width = 64.0;
    let tower_kind_height = 32.0;
    for (i, tower_spec) in TOWER_SPECS.iter().enumerate() {
        let tower_kind_handle = meshes.add(Rectangle::new(tower_kind_width, tower_kind_height));
        let bound_rect = Rect {
            min: Vec2 {
                x: -(WIN_W as f32) / 2.0,
                y: 200.0 + i as f32 * 48.0,
            },
            max: Vec2 {
                x: -(WIN_W as f32) / 2.0 + tower_kind_width,
                y: 200.0 + i as f32 * 48.0 + tower_kind_height,
            },
        };
        commands.spawn((
            TowerKind(i as u8),
            Mesh2d(tower_kind_handle),
            MeshMaterial2d(materials.add(tower_spec.color)),
            Transform::from_xyz(
                bound_rect.min.x + tower_kind_width / 2.0,
                bound_rect.min.y + tower_kind_height / 2.0,
                0.0,
            ),
            Bounds(bound_rect),
        ));
    }
}

fn update_enemy_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Enemy>>,
    time: Res<Time>,
    game_map: Res<GameMap>,
    mut game_state: ResMut<GameState>,
) {
    for (entity, mut transform) in &mut query {
        if path_completed(&transform.translation.xy(), &game_map.path) {
            commands.entity(entity).despawn();
            game_state.base_life -= 5;
        } else {
            let next_pos = calculate_next_position_on_path(
                &transform.translation.xy(),
                &game_map.path,
                100.0 * time.delta_secs(),
            );
            transform.translation.x = next_pos.x;
            transform.translation.y = next_pos.y;
        }
    }
}

fn update_enemy_spawns(
    mut commands: Commands,
    time: Res<Time>,
    game_map: Res<GameMap>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if enemy_spawn_timer.0.tick(time.delta()).just_finished() {
        let shapes = [meshes.add(Circle::new(ENEMY_SIZE_RADIUS))];
        let spawn_point = game_map.path[0];

        for shape in shapes.into_iter() {
            let color = Color::Srgba(Srgba::new(0.2, 0.8, 0.6, 1.0));
            commands
                .spawn((
                    Enemy,
                    Health::new(100),
                    Mesh2d(shape),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_xyz(spawn_point.x, spawn_point.y, 0.0),
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
    }
}

fn update_shooting(
    mut commands: Commands,
    towers: Query<(&Transform, &Tower), With<Tower>>,
    mut enemies: Query<(Entity, &Transform, &mut Health, &Children), With<Enemy>>,
    mut enemy_health_text: Query<&mut Text2d>,
    time: Res<Time>,
    mut shooting_timer: ResMut<ShootingTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for timer in &mut shooting_timer.0 {
        timer.tick(time.delta());
    }

    let mut rng = rand::rng();
    for (tower_transform, tower) in towers {
        if !shooting_timer.0[tower.0 as usize].just_finished() {
            continue;
        }

        let mut reachable_enemy = enemies
            .iter_mut()
            .filter(|(_, transform, _, _)| {
                transform
                    .translation
                    .xy()
                    .distance(tower_transform.translation.xy())
                    <= TOWER_SPECS[tower.0 as usize].distance
            })
            .collect::<Vec<_>>();

        if reachable_enemy.is_empty() {
            continue;
        }

        let random_index = 0..reachable_enemy.len();
        let random_enemy = reachable_enemy
            .get_mut(rng.random_range(random_index))
            .unwrap();

        random_enemy.2.current -= TOWER_SPECS[tower.0 as usize].damage;
        for enemy_child in random_enemy.3 {
            let mut text = enemy_health_text.get_mut(*enemy_child).unwrap();
            *text = Text2d::new(format!("{:.1?}%", random_enemy.2.percentage() * 100.0));
        }

        let line_handle = meshes.add(Segment2d::new(
            Vec2::default(),
            random_enemy.1.translation.xy() - tower_transform.translation.xy(),
        ));
        commands.spawn((
            Bullet,
            LifeSpan(0.2),
            Mesh2d(line_handle),
            MeshMaterial2d(materials.add(Color::Srgba(Srgba::new(0.8, 0.2, 0.1, 1.0)))),
            Transform::from_xyz(
                tower_transform.translation.x,
                tower_transform.translation.y,
                0.0,
            ),
        ));
    }
}

fn update_score(mut query: Query<&mut TextSpan, With<ScoreText>>, game_state: Res<GameState>) {
    for mut span in &mut query {
        **span = format!("Health: {}", game_state.base_life);
    }
}

fn update_detect_tower_picking(
    mut commands: Commands,
    selectable_towers: Query<(&Bounds, &TowerKind), With<TowerKind>>,
    mut mouse_event: MessageReader<MouseButtonInput>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for mouse_button_input in mouse_event.read() {
        if mouse_button_input.button == MouseButton::Left
            && mouse_button_input.state == ButtonState::Pressed
        {
            for (selectable_tower_bound, tower_kind) in selectable_towers {
                if let Some(cursor_pos) = window.cursor_position() {
                    let (cam, cam_transform) = *camera;
                    let cursor_world_pos =
                        cam.viewport_to_world_2d(cam_transform, cursor_pos).unwrap();

                    if selectable_tower_bound.0.contains(cursor_world_pos) {
                        let mesh_handle = meshes.add(Circle::new(TOWER_SIZE_RADIUS));
                        let reach_mesh_handle =
                            meshes.add(Circle::new(TOWER_SPECS[tower_kind.0 as usize].distance));
                        commands
                            .spawn((
                                TowerCandidate(tower_kind.0),
                                Dragged,
                                Mesh2d(mesh_handle),
                                MeshMaterial2d(
                                    materials.add(TOWER_SPECS[tower_kind.0 as usize].color),
                                ),
                                Transform::from_xyz(cursor_world_pos.x, cursor_world_pos.y, 0.0),
                            ))
                            .with_child((
                                Mesh2d(reach_mesh_handle),
                                MeshMaterial2d(materials.add(Color::Srgba(Srgba {
                                    red: 1.0,
                                    green: 1.0,
                                    blue: 1.0,
                                    alpha: 0.1,
                                }))),
                                Transform::from_xyz(0.0, 0.0, -1.0),
                            ));
                    }
                }
            }
        }
    }
}

fn update_drop_tower(
    mut commands: Commands,
    candidate: Single<(Entity, &TowerCandidate), With<TowerCandidate>>,
    mut mouse_event_reader: MessageReader<MouseButtonInput>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in mouse_event_reader.read() {
        if event.button == MouseButton::Left && event.state == ButtonState::Released {
            let cursor_pos = window.cursor_position().unwrap();
            let (cam, cam_transform) = *camera;
            let cursor_world_pos = cam.viewport_to_world_2d(cam_transform, cursor_pos).unwrap();
            let (candidate_entity, candidate) = *candidate;
            commands.entity(candidate_entity).despawn();
            spawn_tower(
                &mut commands,
                &mut meshes,
                &mut materials,
                cursor_world_pos,
                candidate.0,
            );
        }
    }
}

fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
) {
    for mut span in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
            && let Some(value) = fps.smoothed()
        {
            **span = format!("{value:.2}");
        }
    }
}

fn update_game_state(mut game_state: ResMut<GameState>, time: Res<Time>) {
    let spawn_kinds = game_state.waves.update(time.delta());
    if !spawn_kinds.is_empty() {
        println!("Spawn: {:?}", spawn_kinds);
    }
}

pub(crate) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let shooting_timers = TOWER_SPECS
            .iter()
            .map(|spec| Timer::from_seconds(spec.shooting_freq_secs, TimerMode::Repeating))
            .collect();

        app.insert_resource(GameState {
            base_life: 100,
            waves: Waves::load(),
        })
        .insert_resource(GameMap {
            path: vec![
                (-200.0, -200.0).into(),
                (-200.0, 0.0).into(),
                (200.0, 0.0).into(),
                (200.0, 200.0).into(),
            ],
        })
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .insert_resource(ShootingTimer(shooting_timers))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                (
                    update_enemy_spawns,
                    update_enemy_movement,
                    update_shooting,
                    update_life_span,
                    update_health,
                )
                    .chain(),
                update_score,
                update_detect_tower_picking,
                update_move_dragged_objects,
                update_drop_tower,
                update_fps_text,
                update_deletables,
                update_game_state,
            ),
        );
    }
}
