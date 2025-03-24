use bevy::input::gamepad::GamepadAxisChangedEvent;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_framepace::FramepacePlugin;
use bevy_obj::ObjPlugin;
use monarchs::debug_tools::*;
use monarchs::world::WorldPlugin;
use monarchs::{GameState, LookDirection};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(FramepacePlugin)
        .add_plugins(ObjPlugin)
        .add_plugins(DebugTools)
        .add_plugins(WorldPlugin)
        .init_state::<GameState>()
        .add_systems(Startup, (setup_camera, setup_player))
        .add_systems(Update, (update_camera, change_body, update_look_gamepad))
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
            DebugShowAxes,
            Visibility::Inherited,
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
            Visibility::Visible,
            DebugShowLookingDir,
        ))
        .add_children(&mut [body_cube, body_donut]);
}

fn update_camera(
    mut player_camera: Single<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<PlayerCamera>)>,
) {
    player_camera.translation = player.translation + Vec3::new(-5.0, 0.0, 2.5);
    player_camera.look_at(player.translation, Dir3::Z);
    player_camera.translation += Vec3::new(0.0, -0.5, 0.5);
}

fn change_body(
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

fn update_look_gamepad(
    mut q_player: Query<&mut LookDirection, With<Player>>,
    q_controller: Single<&Gamepad>,
    time: Res<Time>,
) {
    let dead_zone = 0.1;

    let x = q_controller.right_stick().x;
    let y = q_controller.right_stick().y;

    let mut rotation = Vec2::ZERO;
    if x.abs() > dead_zone || y.abs() > dead_zone {
        rotation = Vec2::new(x, y);
    }


}
