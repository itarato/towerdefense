mod math;

use bevy::prelude::*;

#[derive(Resource)]
struct GameState {
    score: u32,
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

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

fn update_enemy_movement(mut query: Query<&mut Transform, With<Enemy>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.translation.x += 100.0 * time.delta_secs();
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

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameMap {
            path: vec![
                (-200.0, -200.0).into(),
                (-200.0, 0.0).into(),
                (200.0, 0.0).into(),
                (200.0, 200.0).into(),
            ],
        })
        .insert_resource(TowerSpawnTimer(Timer::from_seconds(1.0, TimerMode::Once)))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_enemy_spawns, update_enemy_movement).chain());
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
