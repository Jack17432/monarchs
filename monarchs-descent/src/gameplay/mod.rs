mod input;
mod player;

use crate::gameplay::input::*;
use avian3d::prelude::{Collider, ColliderConstructor, ColliderConstructorHierarchy, RigidBody};
use bevy::color::palettes::basic::RED;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use crate::CameraOrder;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(input::plugin)
        .add_plugins(player::plugin);

    app.add_systems(Startup, setup);
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(30.0, 30.0, 30.0).looking_at(Vec3::ZERO, Vec3::Z),
        Camera {
            order: CameraOrder::World.into(),
            ..default()
        },
    ));

    commands.spawn(Actions::<OnFoot>::default());

    commands.spawn((
        Transform::from_xyz(10.0, -10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Z),
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
        SceneRoot(map_handle),
        ColliderConstructorHierarchy::new(None)
            .with_constructor_for_name("Ground", ColliderConstructor::TrimeshFromMesh),
        RigidBody::Static,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: RED.into(),
            ..Default::default()
        })),
        Collider::capsule_endpoints(0.5, Vec3::NEG_Z * 0.5, Vec3::Z * 0.5),
        Transform::from_xyz(0.0, 0.0, 15.0),
        RigidBody::Dynamic,
    ));
}
