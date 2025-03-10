use bevy::prelude::*;

use verlet::{Velocity, VerletIntegrationPlugin, VerletObject};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(VerletIntegrationPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_running)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let white_matl = materials.add(Color::WHITE);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(white_matl.clone()),
        Velocity::new(Vec3::new(4.0, 2.0, 0.0)),
        VerletObject,
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
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    ));
}

fn toggle_running(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<verlet::State>>,
    mut next_state: ResMut<NextState<verlet::State>>,
) {
    if input.just_pressed(KeyCode::Space) {
        next_state.set(match current_state.get() {
            verlet::State::Running => verlet::State::Paused,
            verlet::State::Paused => verlet::State::Running,
        });
    }
}