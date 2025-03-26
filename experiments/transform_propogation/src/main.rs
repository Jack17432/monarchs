use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_gizmo, update_obj, update_camera))
        .run();
}

#[derive(Component)]
struct Object;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn(Camera3d::default());

    let child_a = commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
        ))
        .id();

    commands
        .spawn((Object, Transform::default()))
        .add_child(child_a);
}

fn draw_gizmo(
    mut gizmos: Gizmos,
    query: Query<&GlobalTransform>,
) {
    for global_transform in &query {
        let length = 2.0;
        gizmos.axes(*global_transform, length);
    }
}

fn update_obj(
    mut q_object: Single<&mut Transform, With<Object>>,
) {
    q_object.translation += Vec3::new(1.0, 1.0, 0.0);
}

fn update_camera(
    q_object: Single<&Transform, (With<Object>, Without<Camera3d>)>,
    mut q_cam: Single<&mut Transform, With<Camera3d>>,
) {
    q_cam.translation = q_object.translation + Vec3::new(-2.0, -2.0, 3.0);
    q_cam.look_at(q_object.translation, Vec3::Z);
}
