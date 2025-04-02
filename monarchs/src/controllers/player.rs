use crate::config::UserGamepadConfig;
use crate::core::physics::LinerVelocity;
use crate::views::player_camera::PlayerCameraInfo;
use crate::void_born::souls::{
    BoundToVessel, ChangeDir, ChangeNextVesselEvent, NextVessel, VesselSwapEvent,
};
use bevy::prelude::*;

#[derive(Default)]
pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_move_gamepad,
                update_look_gamepad,
                update_vessel_gamepad,
            ),
        );
    }
}

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct PlayerControlled;

fn update_move_gamepad(
    mut q_player: Query<(&mut LinerVelocity, &PlayerCameraInfo), With<PlayerControlled>>,
    q_controller: Option<Single<&Gamepad>>,
    user_gamepad_config: Res<UserGamepadConfig>,
    time: Res<Time>,
) {
    let Some(q_controller) = q_controller else {
        return;
    };

    let [x, y] = (user_gamepad_config.apply_left_stick_config(q_controller.left_stick())
        * time.delta_secs())
    .to_array();

    let (mut lin_vel, look_direction) = q_player.single_mut();

    let y_look_amount = look_direction.0.to_euler(EulerRot::XYZ).2;

    let change_by = Quat::from_rotation_z(y_look_amount) * Vec3::new(y, -x, 0.0);
    lin_vel.0 += change_by * 60.0 * time.delta_secs();
}

fn update_look_gamepad(
    mut q_player: Single<&mut PlayerCameraInfo, With<PlayerControlled>>,
    q_controller: Option<Single<&Gamepad>>,
    user_gamepad_config: Res<UserGamepadConfig>,
    time: Res<Time>,
) {
    let Some(q_controller) = q_controller else {
        return;
    };

    let [x, y] = (user_gamepad_config.apply_right_stick_config(q_controller.right_stick())
        * time.delta_secs())
    .to_array();

    let yaw = Quat::from_rotation_z(-x);
    let pitch = Quat::from_rotation_y(-y);

    let new_rotation = yaw * q_player.0 * pitch;

    q_player.0 = new_rotation;
}

fn update_vessel_gamepad(
    gamepad: Option<Single<&Gamepad>>,
    mut e_vessel_swap: EventWriter<VesselSwapEvent>,
    mut e_vessel_next_change: EventWriter<ChangeNextVesselEvent>,
    player_soul: Single<(Entity, &BoundToVessel, &NextVessel), With<Player>>,
) {
    let Some(gamepad) = gamepad else {
        return;
    };
    let (soul, curr, next) = player_soul.into_inner();

    if gamepad.just_pressed(GamepadButton::DPadDown) {
        info!("Swapping vessel");
        e_vessel_swap.send(VesselSwapEvent(soul, curr.0, next.0));
    }

    if gamepad.just_pressed(GamepadButton::DPadRight) {
        info!("Changing next Forwards");
        e_vessel_next_change.send(ChangeNextVesselEvent(soul, ChangeDir::Forward));
    }
    if gamepad.just_pressed(GamepadButton::DPadLeft) {
        info!("Changing next Backwards");
        e_vessel_next_change.send(ChangeNextVesselEvent(soul, ChangeDir::Backward));
    }
}
