use crate::gameplay::input::Rotate;
use crate::gameplay::player::{Player, PlayerCameraTarget};
use crate::CameraOrder;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_player_camera)
        .add_observer(move_camera);

    app.add_systems(Update, sync_camera_to_player_transform);
}

#[derive(Default, Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Transform, Visibility)]
pub struct PlayerCamera;

fn spawn_player_camera(_trigger: Trigger<OnAdd, PlayerCameraTarget>, mut commands: Commands) {
    commands
        .spawn((
            Name::new("Camera Player"),
            PlayerCamera,
            Transform::default().looking_at(Vec3::X, Vec3::Y),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Camera World"),
                Camera3d::default(),
                Camera {
                    order: CameraOrder::World.into(),
                    ..default()
                },
            ));

            parent.spawn((
                Name::new("Camera ViewModel"),
                Camera3d::default(),
                Camera {
                    order: CameraOrder::ViewModel.into(),
                    ..default()
                },
            ));
        });
}

fn move_camera(
    trigger: Trigger<Fired<Rotate>>,
    mut transform: Single<&mut Transform, With<PlayerCamera>>,
) {
    let delta = trigger.value;

    if delta != Vec2::ZERO {
        let delta_yaw = delta.x;
        let delta_pitch = delta.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

fn sync_camera_to_player_transform(
    mut player_camera: Single<
        &mut Transform,
        (
            With<PlayerCamera>,
            Without<Player>,
            Without<PlayerCameraTarget>,
        ),
    >,
    player: Single<
        &Transform,
        (
            With<Player>,
            Without<PlayerCamera>,
            Without<PlayerCameraTarget>,
        ),
    >,
    player_camera_target: Single<
        &Transform,
        (
            With<PlayerCameraTarget>,
            Without<Player>,
            Without<PlayerCamera>,
        ),
    >,
) {
    player_camera.translation = player.translation + player_camera_target.translation;
}
