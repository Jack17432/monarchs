use crate::{LookDirection, Player};
use bevy::prelude::*;

#[derive(Component)]
#[require(Camera3d)]
pub struct PlayerCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, player_setup_camera)
            .add_systems(PostUpdate, update_camera);
    }
}

fn player_setup_camera(mut commands: Commands, q_player: Single<Entity, With<Player>>) {
    let player_camera = commands.spawn(PlayerCamera).id();
    commands
        .get_entity(*q_player)
        .unwrap()
        .add_child(player_camera);
}

fn update_camera(
    mut player_camera: Single<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    player: Single<&LookDirection, (With<Player>, Without<PlayerCamera>)>,
) {
    let look_direction = *player;

    player_camera.translation = look_direction.0 * Vec3::new(-5.0, -0.5, 3.0);

    let forward = (look_direction.0 * Vec3::X).normalize();
    player_camera.look_at(forward, Dir3::Z);
}
