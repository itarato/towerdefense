use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Dragged;

pub(crate) fn update_move_dragged_objects(
    mut query: Query<&mut Transform, With<Dragged>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    for mut dragged_obj in &mut query {
        if let Some(cursor_pos) = window.cursor_position() {
            let (cam, cam_transform) = *camera;
            let cursor_world_pos = cam.viewport_to_world_2d(cam_transform, cursor_pos).unwrap();

            dragged_obj.translation.x = cursor_world_pos.x;
            dragged_obj.translation.y = cursor_world_pos.y;
        }
    }
}
