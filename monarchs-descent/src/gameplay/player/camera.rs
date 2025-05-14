use crate::gameplay::input::Rotate;
use crate::gameplay::player::Player;
use crate::{CameraOrder, DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_enhanced_input::events::Fired;
use std::f32::consts::FRAC_PI_2;
use crate::gameplay::items::inventory::{Equipped, EquippedItem};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(move_camera_with_look)
        .add_observer(spawn_player_camera)
        .add_observer(swap_held_item);

    app.add_systems(Update, sync_camera_to_player_transform);
}

#[derive(Default, Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Transform, Visibility)]
pub struct PlayerCamera;

pub fn spawn_player_camera(_trigger: Trigger<OnAdd, PlayerCameraTarget>, mut commands: Commands) {
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
                RenderLayers::layer(DEFAULT_RENDER_LAYER),
            ));

            parent.spawn((
                Name::new("Camera ViewModel"),
                Camera3d::default(),
                Camera {
                    order: CameraOrder::ViewModel.into(),
                    ..default()
                },
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ));
        });
}

pub fn move_camera_with_look(
    trigger: Trigger<Fired<Rotate>>,
    mut camera_transform: Single<&mut Transform, With<PlayerCamera>>,
    mut player_transform: Single<&mut Transform, (With<Player>, Without<PlayerCamera>)>,
) {
    let delta = trigger.value;
    if delta != Vec2::ZERO {
        let delta_yaw = delta.x;
        let delta_pitch = delta.y;

        let (yaw, pitch, roll) = camera_transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        player_transform.rotation = Quat::from_rotation_y(yaw);
    }
}

pub fn sync_camera_to_player_transform(
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

#[derive(Default, Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerCameraTarget;

fn swap_held_item(
    mut commands: Commands,
    trigger: Trigger<Changed<EquippedItem>>,
    player: Single<Entity, With<Player>>,
    view_model: Single<(Entity, &mut Children), With<PlayerCamera>>,
    
) {
    let (view_model_entity, children) = view_model.into_inner();
    if children.contains()
}
