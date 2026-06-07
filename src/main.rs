mod dragged;
mod health;
mod math;

use crate::{
    dragged::{Dragged, update_move_dragged_objects},
    health::{Health, update_health},
    math::{calculate_next_position_on_path, path_completed},
};
use bevy::{
    color::palettes::css::{RED, YELLOW},
    input::{ButtonState, mouse::MouseButtonInput},
    prelude::*,
};
use rand::prelude::*;

const WIN_W: u32 = 1024;
const WIN_H: u32 = 768;

#[derive(Component)]
struct Bounds(Rect);

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

#[derive(Component)]
struct TowerKind;

#[derive(Component)]
struct TowerCandidate;

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

    let mesh_handle = meshes.add(Circle::new(48.0));
    let color = Color::Srgba(Srgba::new(0.8, 0.2, 0.6, 1.0));
    commands.spawn((
        Tower,
        Mesh2d(mesh_handle),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(-100.0, 200.0, 0.0),
    ));

    // Spawnable towers:
    let tower_kind_width = 64.0;
    let tower_kind_height = 32.0;
    let tower_kind_handles = [meshes.add(Rectangle::new(tower_kind_width, tower_kind_height))];
    for mesh_handle in tower_kind_handles.into_iter() {
        let color = Color::Srgba(Srgba::new(0.8, 0.2, 0.6, 1.0));
        let bound_rect = Rect {
            min: Vec2 {
                x: -(WIN_W as f32) / 2.0,
                y: 200.0,
            },
            max: Vec2 {
                x: -(WIN_W as f32) / 2.0 + tower_kind_width,
                y: 200.0 + tower_kind_height,
            },
        };
        commands.spawn((
            TowerKind,
            Mesh2d(mesh_handle),
            MeshMaterial2d(materials.add(color)),
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
    mut enemy_spawn_timer: ResMut<TowerSpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if enemy_spawn_timer.0.tick(time.delta()).just_finished() {
        let shapes = [meshes.add(Circle::new(32.0))];
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
                    TextColor(YELLOW.into()),
                    Transform::from_xyz(-30.0, -60.0, 0.0),
                    EnemyHealthText,
                ));
        }
    }
}

fn update_shooting(
    mut commands: Commands,
    towers: Query<&Transform, With<Tower>>,
    enemies: Query<(Entity, &Transform, &mut Health, &Children), With<Enemy>>,
    mut enemy_health_text: Query<&mut Text2d>,
    time: Res<Time>,
    mut shooting_timer: ResMut<ShootingTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !shooting_timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let mut rng = rand::rng();
    let mut enemies_vec = enemies.into_iter().collect::<Vec<_>>();

    if enemies_vec.is_empty() {
        return;
    }

    if let Some(tower_transform) = towers.iter().next() {
        let random_index = 0..enemies_vec.len();
        let random_enemy = enemies_vec.get_mut(rng.random_range(random_index)).unwrap();

        random_enemy.2.current -= 25;
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

fn update_detect_tower_picking(
    mut commands: Commands,
    selectable_towers: Query<&Bounds, With<TowerKind>>,
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
            for selectable_tower_bound in selectable_towers {
                if let Some(cursor_pos) = window.cursor_position() {
                    let (cam, cam_transform) = *camera;
                    let cursor_world_pos =
                        cam.viewport_to_world_2d(cam_transform, cursor_pos).unwrap();

                    if selectable_tower_bound.0.contains(cursor_world_pos) {
                        let mesh_handle = meshes.add(Circle::new(48.0));
                        let color = Color::Srgba(Srgba::new(0.8, 0.2, 0.6, 1.0));
                        commands.spawn((
                            TowerCandidate,
                            Dragged,
                            Mesh2d(mesh_handle),
                            MeshMaterial2d(materials.add(color)),
                            Transform::from_xyz(cursor_world_pos.x, cursor_world_pos.y, 0.0),
                        ));
                    }
                }
            }
        }
    }
}

fn update_drop_tower(
    mut commands: Commands,
    candidate: Single<Entity, With<TowerCandidate>>,
    mut mouse_event_reader: MessageReader<MouseButtonInput>,
) {
    for event in mouse_event_reader.read() {
        if event.button == MouseButton::Left && event.state == ButtonState::Released {
            commands.entity(*candidate).despawn();
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
                0.5,
                TimerMode::Repeating,
            )))
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
                ),
            );
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WIN_W, WIN_H).into(),
                title: "Tower Defense".into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(GamePlugin)
        .run();
}
