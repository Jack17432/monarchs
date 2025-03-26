use bevy::input::gamepad::{GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadInput};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_framepace::FramepacePlugin;
use bevy_obj::ObjPlugin;
use monarchs::core::*;
use monarchs::debug::*;
use monarchs::environment::*;
use monarchs::{GameState, LookDirection};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        // .add_plugins(FramepacePlugin)
        .add_plugins(ObjPlugin)
        .add_plugins(DebugTools)
        .add_plugins(WorldPlugin)
        // .add_plugins(PhysicsPlugin)
        .init_state::<GameState>()
        .add_systems(Startup, (setup_camera, setup_player))
        .add_systems(
            Update,
            (
                update_camera,
                change_body_keyboard,
                change_body_controller,
                update_look_gamepad,
                move_controller,
            ),
        )
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerBody {
    active: bool,
}

#[derive(Component)]
#[require(Camera3d)]
struct PlayerCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn(PlayerCamera);
}

fn setup_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let body_cube = commands
        .spawn((
            PlayerBody { active: true },
            Visibility::Inherited,
            DebugShowAxes,
            Mesh3d(asset_server.load::<Mesh>("meshes/cube.obj")),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("textures/Grass.png")),
                ..default()
            })),
        ))
        .id();

    let body_donut = commands
        .spawn((
            PlayerBody { active: false },
            Visibility::Hidden,
            DebugShowAxes,
            SceneRoot(asset_server.load("meshes/donut.glb#Scene0")),
        ))
        .id();

    commands
        .spawn((
            Player,
            LookDirection(Quat::IDENTITY),
            Transform::default(),
            PhysicsObject,
            Visibility::Visible,
            DebugShowAxes,
            DebugShowLookingDir,
        ))
        .add_children(&mut [body_cube, body_donut]);
}

fn update_camera(
    mut player_camera: Single<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    player: Single<(&GlobalTransform, &LookDirection), (With<Player>, Without<PlayerCamera>)>,
) {
    let (global_transform, look_direction) = *player;

    player_camera.translation =
        global_transform.translation() + (look_direction.0 * Vec3::new(-5.0, -0.5, 3.0));

    let forward = (look_direction.0 * Vec3::X).normalize();
    player_camera.look_at(global_transform.translation() + forward, Dir3::Z);
}

fn change_body_keyboard(
    player: Single<&Children, (With<Player>, Without<PlayerBody>)>,
    mut body: Query<(&mut Visibility, &mut PlayerBody), Without<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyQ) {
        for &child in player.iter() {
            let (mut visibility, mut body) = body.get_mut(child).unwrap();

            if body.active {
                *visibility = Visibility::Hidden;
                body.active = false;
            } else {
                *visibility = Visibility::Inherited;
                body.active = true;
            }
        }
    }
}

fn change_body_controller(
    player: Single<&Children, (With<Player>, Without<PlayerBody>)>,
    mut body: Query<(&mut Visibility, &mut PlayerBody), Without<Player>>,
    controller: Single<&Gamepad>,
) {
    if controller.just_pressed(GamepadButton::DPadDown) {
        for &child in player.iter() {
            let (mut visibility, mut body) = body.get_mut(child).unwrap();

            if body.active {
                *visibility = Visibility::Hidden;
                body.active = false;
            } else {
                *visibility = Visibility::Inherited;
                body.active = true;
            }
        }
    }
}

fn move_controller(
    mut q_player: Query<(&mut Transform, &LookDirection), With<Player>>,
    q_controller: Single<&Gamepad>,
) {
    let dead_zone = 0.1;

    let (mut transform, look_direction) = q_player.single_mut();
    let mut y = q_controller.left_stick().x;
    let mut x = q_controller.left_stick().y;

    // if x.abs() < dead_zone && y.abs() < dead_zone {
    //     return;
    // }
    //
    // if x.abs() < dead_zone {
    //     x = 0.0;
    // }
    //
    // if y.abs() < dead_zone {
    //     y = 0.0;
    // }

    let y_look_amount = look_direction.0.to_euler(EulerRot::XYZ).2;

    let change_by = Quat::from_rotation_z(y_look_amount) * Vec3::new(x, -y, 0.0);
    transform.translation += change_by;

    // info!(entity = ?entity, change = ?change_by, transform = ?transform);
}

fn update_look_gamepad(
    mut q_player: Single<&mut LookDirection, With<Player>>,
    q_controller: Single<&Gamepad>,
    time: Res<Time>,
) {
    let dead_zone = 0.1;
    let sensitivity = 0.5;

    let mut x = q_controller.right_stick().x;
    let mut y = q_controller.right_stick().y;

    if x.abs() < dead_zone && y.abs() < dead_zone {
        return;
    }

    if x.abs() < dead_zone {
        x = 0.0;
    }

    if y.abs() < dead_zone {
        y = 0.0;
    }

    x *= sensitivity * time.delta_secs();
    y *= sensitivity * time.delta_secs();

    let yaw = Quat::from_rotation_z(-x);
    let pitch = Quat::from_rotation_y(-y);

    let new_rotation = yaw * q_player.0 * pitch;

    q_player.0 = new_rotation;
}
