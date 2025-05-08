pub(in crate::gameplay) mod controller;
mod interact;
mod inventory;

use crate::gameplay::items::inventory::Inventory;
use crate::gameplay::player::controller::PlayerControllerBundle;
use crate::gameplay::player::interact::InteractionRange;
use avian3d::prelude::Collider;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(inventory::plugin)
        .add_plugins(controller::plugin)
        .add_plugins(interact::plugin);

    app.add_systems(Startup, spawn_test_player);
}

#[derive(Default, Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Default, Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerCameraTarget;

fn spawn_test_player(mut commands: Commands) {
    let player = commands
        .spawn((
            Name::new("Player"),
            Player,
            Transform::from_xyz(0.0, 15.0, 0.0),
            PlayerControllerBundle::new(Collider::capsule_endpoints(
                0.5,
                Vec3::NEG_Y * 0.5,
                Vec3::Y * 0.5,
            )),
            InteractionRange(5.0),
            Inventory::new(30),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Player Camera Target"),
                PlayerCameraTarget,
                Transform::from_xyz(0.0, 0.5, 0.0),
            ));
        })
        .id();
}
