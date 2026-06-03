mod math;

use crate::math::{calculate_next_position_on_path, path_completed};
use bevy::{
    color::palettes::css::{RED, YELLOW},
    core_pipeline::core_3d::Transmissive3d,
    prelude::*,
};
use rand::prelude::*;

#[derive(Resource)]
struct GameState {
    base_life: i32,
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

#[derive(Resource)]
struct ShootingTimer(Timer);

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct EnemyHealthText;

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct LifeSpan(f32);

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

    let mesh_handles = [meshes.add(Circle::new(48.0))];

    for mesh_handle in mesh_handles.into_iter() {
        let color = Color::Srgba(Srgba::new(0.8, 0.2, 0.6, 1.0));
        commands.spawn((
            Tower,
            Mesh2d(mesh_handle),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(-100.0, 200.0, 0.0),
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
    mut tower_spawn_timer: ResMut<TowerSpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if tower_spawn_timer.0.tick(time.delta()).just_finished() {
        let shapes = [meshes.add(Circle::new(32.0))];
        let spawn_point = game_map.path[0];

        for shape in shapes.into_iter() {
            let color = Color::Srgba(Srgba::new(0.2, 0.8, 0.6, 1.0));
            let enemy_entity = commands
                .spawn((
                    Enemy,
                    Mesh2d(shape),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_xyz(spawn_point.x, spawn_point.y, 0.0),
                ))
                .id();

            let text_entity = commands
                .entity(enemy_entity)
                .with_child((Text::default(), Transform::from_xyz(-30.0, -60.0, 0.0)))
                .id();
            commands.entity(text_entity).with_child((
                EnemyHealthText,
                TextSpan::new("100%"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(YELLOW.into()),
            ));
        }
    }
}

fn update_shooting(
    mut commands: Commands,
    towers: Query<&Transform, With<Tower>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    time: Res<Time>,
    mut shooting_timer: ResMut<ShootingTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !shooting_timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let mut rng = rand::rng();
    let enemies_vec = enemies.iter().collect::<Vec<_>>();

    if enemies_vec.is_empty() {
        return;
    }

    if let Some(tower_transform) = towers.iter().next() {
        let random_enemy = enemies_vec[rng.random_range(0..enemies_vec.len())];
        let enemy_transform = random_enemy.1;

        let line_handle = meshes.add(Segment2d::new(
            Vec2::default(),
            enemy_transform.translation.xy() - tower_transform.translation.xy(),
        ));
        commands.spawn((
            Bullet,
            LifeSpan(0.3),
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

fn update_life_span(
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
            .insert_resource(TowerSpawnTimer(Timer::from_seconds(
                1.0,
                TimerMode::Repeating,
            )))
            .insert_resource(ShootingTimer(Timer::from_seconds(
                1.0,
                TimerMode::Repeating,
            )))
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    update_enemy_spawns,
                    update_enemy_movement,
                    update_shooting,
                    update_life_span,
                    update_score,
                )
                    .chain(),
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
