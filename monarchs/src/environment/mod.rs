use bevy::prelude::*;
use crate::core::collision::Collider;

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

    for x in -10..=10 {
        for y in -10..=10 {
            cube_batch.push((
                Transform::from_xyz(x as f32, y as f32, -1.0),
                Collider::from_cuboid(Cuboid::new(1.0, 1.0, 1.0)),
                Mesh3d(cube_mesh.clone()),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color_texture: Some(grass_texture.clone()),
                    ..default()
                })),
            ));
        }
    }
    cube_batch.push((
        Transform::from_xyz(3.0, 3.0, 1.0),
        Collider::from_cuboid(Cuboid::new(1.0, 1.0, 1.0)),
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
