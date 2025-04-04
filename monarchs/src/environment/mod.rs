use crate::core::physics::{Collider, PhysicsBodyType};
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_flat);
    }
}

fn create_flat(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let cube_mesh = asset_server.load::<Mesh>("meshes/cube.obj");
    let grass_texture = asset_server.load("textures/Grass.png");

    let mut cube_batch = Vec::new();

    cube_batch.push((
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(20.0, 20.0, 1.0)),
        PhysicsBodyType::Static,
        Collider::from_cuboid(10.0, 10.0, 0.5),
        Mesh3d(cube_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(grass_texture.clone()),
            ..default()
        })),
    ));
    commands.spawn_batch(cube_batch);

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
