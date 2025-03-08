use std::f32::consts::*;

use bevy::color::palettes::basic::{AQUA, GREEN, RED, WHITE};
use bevy::ecs::observer::TriggerTargets;
use bevy::math::bounding::Aabb3d;
use bevy::picking::mesh_picking::ray_cast::{ray_aabb_intersection_3d, ray_mesh_intersection};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::render::render_resource::TextureViewDimension::Cube;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, ray_cast)
        .run();
}

fn ray_cast(
    mut gizmos: Gizmos,
    team_a_query: Query<(), With<ATeam>>,
    mut ray_cast: MeshRayCast,
) {
    gizmos.sphere(Isometry3d::IDENTITY, 0.1, WHITE);

    let ray = Ray3d::new(Vec3::ZERO, Dir3::X);

    let mut config = RayCastSettings::default();

    let binding = |entity: Entity| { team_a_query.contains(entity) };
    config.filter = &binding;

    let mut rays = Vec::new();
    let phi: f32 = PI * ((5.0_f32.sqrt()) - 1.0);

    for idx in 0..1000 {
        let y = 1.0 - (idx as f32 / (1000.0 - 1.0)) * 2.0;
        let r = 1.0;

        let theta = phi * idx as f32;
        let x = f32::cos(theta) * r;
        let z = f32::sin(theta) * r;

        rays.push(Ray3d::new(Vec3::ZERO, Dir3::from_xyz(x, y, z).unwrap()));
    }

    for ray in rays {
        if let Some((_, hit)) = ray_cast
            .cast_ray(ray, &config)
            .first()
        {
            gizmos.line(Vec3::ZERO, hit.point, GREEN);
        } else {}
    }
}

const SHAPES_X_EXTENT: f32 = 14.0;
const EXTRUSION_X_EXTENT: f32 = 16.0;
const Z_EXTENT: f32 = 5.0;

#[derive(Component)]
struct ATeam;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let white_matl = materials.add(Color::WHITE);

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(white_matl.clone()),
            Transform::from_xyz(
                2.0,
                0.0,
                0.0,
            ),
            ATeam,
        ));

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(white_matl.clone()),
            Transform::from_xyz(
                -2.0,
                0.0,
                0.0,
            ),
        ));

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(white_matl.clone()),
            Transform::from_xyz(
                0.0,
                0.0,
                -2.0,
            ),
            ATeam,
        ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));
}