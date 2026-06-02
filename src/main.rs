mod math;

use bevy::{color::palettes::css::RED, prelude::*};

use crate::math::{calculate_next_position_on_path, path_completed};

#[derive(Resource)]
struct GameState {
    base_life: u32,
}

#[derive(Resource)]
struct GameMap {
    path: Vec<Vec2>,
}

#[derive(Component)]
struct Tower;

#[derive(Component)]
struct Enemy;

#[derive(Resource)]
struct TowerSpawnTimer(Timer);

#[derive(Component)]
struct ScoreText;

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
            TextSpan::default(),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(RED.into()),
            ScoreText,
        ));

    let shapes = [meshes.add(Circle::new(48.0))];

    for shape in shapes.into_iter() {
        let color = Color::Srgba(Srgba::new(0.8, 0.2, 0.6, 1.0));
        commands.spawn((
            Tower,
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(100.0, 200.0, 0.0),
        ));
    }
}

fn update_enemy_movement(
    mut query: Query<&mut Transform, With<Enemy>>,
    time: Res<Time>,
    game_map: Res<GameMap>,
) {
    for mut transform in &mut query {
        // transform.translation.x += 100.0 * time.delta_secs();
        if !path_completed(&transform.translation.xy(), &game_map.path) {
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
    mut tower_spawn_timer: ResMut<TowerSpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if tower_spawn_timer.0.tick(time.delta()).just_finished() {
        let shapes = [meshes.add(Circle::new(32.0))];
        let spawn_point = game_map.path[0];

        for shape in shapes.into_iter() {
            let color = Color::Srgba(Srgba::new(0.2, 0.8, 0.6, 1.0));
            commands.spawn((
                Enemy,
                Mesh2d(shape),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(spawn_point.x, spawn_point.y, 0.0),
            ));
        }
    }
}

fn update_score(mut query: Query<&mut TextSpan, With<ScoreText>>, game_state: Res<GameState>) {
    for mut span in &mut query {
        **span = format!("Health: {}", game_state.base_life);
    }
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState { base_life: 100 })
            .insert_resource(GameMap {
                path: vec![
                    (-200.0, -200.0).into(),
                    (-200.0, 0.0).into(),
                    (200.0, 0.0).into(),
                    (200.0, 200.0).into(),
                ],
            })
            .insert_resource(TowerSpawnTimer(Timer::from_seconds(1.0, TimerMode::Once)))
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    (update_enemy_spawns, update_enemy_movement).chain(),
                    update_score,
                ),
            );
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1024, 768).into(),
                title: "Tower Defense".into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(GamePlugin)
        .run();
}
