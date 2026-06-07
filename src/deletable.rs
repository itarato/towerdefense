use bevy::{
    input::{ButtonState, mouse::MouseButtonInput},
    prelude::*,
};

use crate::util::Bounds;

#[derive(Component)]
pub(crate) struct Deletable;

pub(crate) fn update_deletables(
    mut commands: Commands,
    query: Query<(Entity, &Bounds), (With<Deletable>, With<Bounds>)>,
    mut mouse_event_reader: MessageReader<MouseButtonInput>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    for event in mouse_event_reader.read() {
        if event.button == MouseButton::Middle && event.state == ButtonState::Pressed {
            let cursor_pos = window.cursor_position().unwrap();
            let (cam, cam_transform) = *camera;
            let cursor_world_pos = cam.viewport_to_world_2d(cam_transform, cursor_pos).unwrap();

            for (entity, bounds) in query {
                if bounds.0.contains(cursor_world_pos) {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
