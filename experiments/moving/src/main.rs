use bevy::color::palettes::basic::AQUA;
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, update_positions).chain())
        .run();
}

#[derive(Component, Debug)]
struct Player;

#[derive(Component, Debug, Default)]
struct Velocity(Vec2);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Player,
        Velocity::default(),
        Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
        MeshMaterial2d(materials.add(Color::from(AQUA))),
        Transform::default(),
    ));
}

fn move_player(
    mut velocity: Single<&mut Velocity, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>
) {
    if keyboard.pressed(KeyCode::KeyW) {
        velocity.0.y = 50.0;
    } else if keyboard.pressed(KeyCode::KeyS) {
        velocity.0.y = -50.0;
    } else {
        velocity.0.y = 0.0;
    }

    if keyboard.pressed(KeyCode::KeyA) {
        velocity.0.x = -50.0;
    } else if keyboard.pressed(KeyCode::KeyD) {
        velocity.0.x = 50.0;
    } else {
        velocity.0.x = 0.0;
    }
}

fn update_positions(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}