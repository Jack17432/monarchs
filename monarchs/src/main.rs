use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy_egui::EguiPlugin;
use bevy_framepace::FramepacePlugin;
use monarchs::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(FramepacePlugin)
        .init_state::<GameState>()
        .add_systems(Startup, (setup_camera, setup_player))
        .add_systems(Update, (update_camera, change_body, draw_axes))
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct ShowAxes;

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let body_a = commands
        .spawn((
            PlayerBody { active: true },
            Visibility::Inherited,
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        ))
        .id();

    let body_b = commands
        .spawn((
            PlayerBody { active: false },
            Visibility::Hidden,
            ShowAxes,
            SceneRoot(asset_server.load("donut.glb#Scene0")),
        ))
        .id();

    commands
        .spawn((Player, Transform::default(), Visibility::Visible))
        .add_children(&mut [body_a, body_b]);
}

fn update_camera(
    mut player_camera: Single<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<PlayerCamera>)>,
) {
    player_camera.look_at(player.translation, Dir3::Z);
    player_camera.translation = player.translation + Vec3::new(0.0, 1.0, 0.5);
}

fn draw_axes(mut gizmos: Gizmos, query: Query<&Transform, With<ShowAxes>>) {
    for &transform in &query {
        let length = 1.0;
        gizmos.axes(transform, length);
    }
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
