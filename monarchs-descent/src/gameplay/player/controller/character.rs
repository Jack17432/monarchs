use crate::gameplay::player::controller::camera::PlayerCamera;
use crate::gameplay::player::Player;
use avian3d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_player_rotation_to_camera);
}

fn update_player_rotation_to_camera(
    camera_transform: Single<&Transform, With<PlayerCamera>>,
    mut player_rotation: Single<&mut Rotation, (With<Player>, Without<PlayerCamera>)>,
) {
    let (_, yaw, _) = camera_transform.rotation.to_euler(EulerRot::XYZ);
}
