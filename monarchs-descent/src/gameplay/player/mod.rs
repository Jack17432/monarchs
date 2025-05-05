pub(in crate::gameplay) mod controller;
mod interaction;
mod inventory;

use crate::gameplay::player::controller::PlayerControllerBundle;
use crate::gameplay::player::interaction::InteractionRange;
use avian3d::prelude::{Collider, LockedAxes, RigidBody};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(inventory::plugin)
        .add_plugins(controller::plugin)
        .add_plugins(interaction::plugin);

    app.add_systems(Startup, spawn_test_player);
}

#[derive(Default, Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Default, Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerCameraTarget;

fn spawn_test_player(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Name::new("Player"),
            Player,
            Transform::from_xyz(0.0, 15.0, 0.0),
            PlayerControllerBundle::new(Collider::capsule_endpoints(
                0.5,
                Vec3::NEG_Y * 0.5,
                Vec3::Y * 0.5,
            )),
            InteractionRange(3.0),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Player Camera Target"),
                PlayerCameraTarget,
                Transform::from_xyz(0.0, 0.5, 0.0),
            ));
        });
}
