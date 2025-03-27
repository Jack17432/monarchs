use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_framepace::FramepacePlugin;
use bevy_obj::ObjPlugin;
use monarchs::core::*;
use monarchs::debug::*;
use monarchs::environment::*;
use monarchs::views::CameraPlugin;
use monarchs::{GameState, LookDirection, Player};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(FramepacePlugin)
        .add_plugins(ObjPlugin)
        .add_plugins(DebugTools)
        .add_plugins(WorldPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(CameraPlugin)
        .init_state::<GameState>()
        .add_systems(Startup, setup_player)
        .add_systems(
            Update,
            (
                change_body_keyboard,
                change_body_controller,
                update_look_gamepad,
                move_controller,
            ),
        )
        .run();
}

#[derive(Component)]
struct PlayerBody {
    active: bool,
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
            Mesh3d(asset_server.load::<Mesh>("meshes/cube.obj")),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("textures/Grass.png")),
                ..default()
            })),
        ))
        .id();
    info!(id = ?body_cube, "Cube entity spawned");

    let body_donut = commands
        .spawn((
            PlayerBody { active: false },
            Visibility::Hidden,
            SceneRoot(asset_server.load("meshes/donut.glb#Scene0")),
        ))
        .id();
    info!(id = ?body_donut, "donut entity spawned");

    commands
        .spawn((
            Player,
            LookDirection(Quat::IDENTITY),
            Visibility::Visible,
            DebugShowAxes,
            DebugShowLookingDir,

            PhysicsBundle {
                transform: Transform::default(),
                velocity: Velocity(Vec3::ZERO),
                acceleration: Acceleration(Vec3::ZERO),
                forces: Forces(Vec3::ZERO),
                damping: Damping(0.995),
                mass: Mass::new(60.0),
            },
        ))
        .add_children(&mut [body_cube, body_donut]);
}

fn change_body_keyboard(
    player: Single<&Children, (With<Player>, Without<PlayerBody>)>,
    mut body: Query<(&mut Visibility, &mut PlayerBody), Without<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyQ) {
        for &child in player.iter() {
            let Ok((mut visibility, mut body)) = body.get_mut(child) else {
                continue;
            };

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
    mut body: Query<(&mut Visibility, &mut PlayerBody), (With<PlayerBody>, Without<Player>)>,
    controller: Single<&Gamepad>,
) {
    if controller.just_pressed(GamepadButton::DPadDown) {
        for &child in player.iter() {
            let Ok((mut visibility, mut body)) = body.get_mut(child) else {
                continue;
            };

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
    q_controller: Option<Single<&Gamepad>>,
) {
    let Some(q_controller) = q_controller else {
        return;
    };

    let dead_zone = 0.1;

    let (mut transform, look_direction) = q_player.single_mut();
    let mut y = q_controller.left_stick().x;
    let mut x = q_controller.left_stick().y;

    if x.abs() < dead_zone && y.abs() < dead_zone {
        return;
    }

    if x.abs() < dead_zone {
        x = 0.0;
    }

    if y.abs() < dead_zone {
        y = 0.0;
    }

    let y_look_amount = look_direction.0.to_euler(EulerRot::XYZ).2;

    let change_by = Quat::from_rotation_z(y_look_amount) * Vec3::new(x, -y, 0.0);
    transform.translation += change_by;
}

fn update_look_gamepad(
    mut q_player: Single<&mut LookDirection, With<Player>>,
    q_controller: Option<Single<&Gamepad>>,
    time: Res<Time>,
) {
    let Some(q_controller) = q_controller else {
        return;
    };

    let dead_zone = 0.1;
    let sensitivity = 2.0;

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
