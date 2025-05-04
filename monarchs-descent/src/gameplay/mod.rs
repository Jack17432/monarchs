mod input;
mod player;

use crate::gameplay::input::*;
use avian3d::prelude::{ColliderConstructor, ColliderConstructorHierarchy, RigidBody};
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(input::plugin)
        .add_plugins(player::plugin);

    app.add_systems(Startup, setup);
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Actions::<PlayerActions>::default());

    commands.spawn((
        Name::new("Light Sun"),
        Transform::from_xyz(10.0, 10.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .build(),
    ));

    let map_handle =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("maps/basic_surface.glb"));

    commands.spawn((
        Name::new("Map Basic surface"),
        SceneRoot(map_handle),
        ColliderConstructorHierarchy::new(None)
            .with_constructor_for_name("Ground", ColliderConstructor::TrimeshFromMesh),
        RigidBody::Static,
    ));

}
