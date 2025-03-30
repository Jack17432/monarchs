use bevy::prelude::*;
use crate::core::{Collider, PhysicsBodyType};

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

    for x in -3..=3 {
        for y in -3..=3 {
            cube_batch.push((
                Transform::from_xyz(x as f32, y as f32, -1.0),
                PhysicsBodyType::Static,
                Collider::from_cuboid(0.5, 0.5, 0.5),
                Mesh3d(cube_mesh.clone()),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color_texture: Some(grass_texture.clone()),
                    ..default()
                })),
            ));
        }
    }
    cube_batch.push((
        Transform::from_xyz(3.0, 3.0, 0.0),
        PhysicsBodyType::Static,
        Collider::from_cuboid(0.5, 0.5, 0.5),
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
