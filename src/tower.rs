use bevy::prelude::*;

use crate::{deletable::Deletable, util::Bounds};

pub(crate) const TOWER_SIZE_RADIUS: f32 = 32.0;
pub(crate) const TOWER_SPECS: [TowerSpecs; 3] = [
    TowerSpecs {
        distance: 100.0,
        damage: 10,
        shooting_freq_secs: 0.1,
        color: Color::Srgba(Srgba::new(0.2, 0.4, 0.6, 1.0)),
    },
    TowerSpecs {
        distance: 200.0,
        damage: 50,
        shooting_freq_secs: 0.6,
        color: Color::Srgba(Srgba::new(0.4, 0.2, 0.6, 1.0)),
    },
    TowerSpecs {
        distance: 300.0,
        damage: 80,
        shooting_freq_secs: 1.2,
        color: Color::Srgba(Srgba::new(0.6, 0.2, 0.4, 1.0)),
    },
];

pub(crate) struct TowerSpecs {
    pub(crate) distance: f32,
    pub(crate) damage: i32,
    pub(crate) shooting_freq_secs: f32,
    pub(crate) color: Color,
}

impl TowerSpecs {
    fn bounds_rect_at_pos(&self, pos: Vec2) -> Rect {
        Rect {
            min: Vec2 {
                x: pos.x - TOWER_SIZE_RADIUS,
                y: pos.y - TOWER_SIZE_RADIUS,
            },
            max: Vec2 {
                x: pos.x + TOWER_SIZE_RADIUS,
                y: pos.y + TOWER_SIZE_RADIUS,
            },
        }
    }
}

#[derive(Component)]
pub(crate) struct Tower(pub(crate) u8);

#[derive(Component)]
pub(crate) struct TowerKind(pub(super) u8);

#[derive(Component)]
pub(crate) struct TowerCandidate(pub(crate) u8);

#[derive(Resource)]
pub(crate) struct ShootingTimer(pub(crate) Vec<Timer>);

pub(crate) fn spawn_tower(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    pos: Vec2,
    kind: u8,
) {
    let mesh_handle = meshes.add(Circle::new(TOWER_SIZE_RADIUS));
    commands.spawn((
        Tower(kind),
        Mesh2d(mesh_handle),
        MeshMaterial2d(materials.add(TOWER_SPECS[kind as usize].color)),
        Transform::from_xyz(pos.x, pos.y, 0.0),
        Deletable,
        Bounds(TOWER_SPECS[kind as usize].bounds_rect_at_pos(pos)),
    ));
}
