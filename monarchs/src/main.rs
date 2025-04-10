use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_framepace::FramepacePlugin;
use bevy_obj::ObjPlugin;
use monarchs::controllers::player::{Player, PlayerControlled};
use monarchs::core::physics::{Collider, PhysicsBodyType};
use monarchs::debug::*;
use monarchs::views::player_camera::*;
use monarchs::void_born::souls::SoulBundle;
use monarchs::void_born::vessels::VesselBundle;
use monarchs::{GameState, MonarchsGamePluginGroup};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Monarchs".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((EguiPlugin, FramepacePlugin, ObjPlugin))
        .add_plugins(MonarchsGamePluginGroup)
        .init_state::<GameState>()
        .add_systems(Startup, setup_player)
        .run();
}

fn setup_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let cube_vessel = commands
        .spawn((
            Transform::from_xyz(-2.0, -2.0, 1.0),
            PhysicsBodyType::Controlled,
            PlayerControlled,
            PlayerCameraInfo(Quat::IDENTITY),
            Mesh3d(asset_server.load::<Mesh>("meshes/cube.obj")),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("textures/Grass.png")),
                ..default()
            })),
            DebugShowAxes,
            DebugCameraPoint,
            DebugCollisionMesh,
        ))
        .insert(VesselBundle::new(
            String::from("Grassius"),
            Some(String::from("Grass block")),
        ))
        .id();

    let donut_vessel = commands
        .spawn((
            Transform::from_xyz(2.0, 2.0, 1.0),
            PhysicsBodyType::Controlled,
            SceneRoot(asset_server.load("meshes/donut.glb#Scene0")),
            DebugShowAxes,
            DebugCameraPoint,
            DebugCollisionMesh,
        ))
        .insert(VesselBundle::new(String::from("Donut"), None))
        .id();

    let std_cube_vessel = commands
        .spawn((
            Transform::from_xyz(2.0, 0.0, 1.0),
            PhysicsBodyType::Controlled,
            Mesh3d(asset_server.load::<Mesh>("meshes/cube.obj")),
            MeshMaterial3d(materials.add(Color::WHITE)),
            DebugShowAxes,
            DebugCameraPoint,
            DebugCollisionMesh,
        ))
        .insert(VesselBundle::new(
            String::from("Std blue cube"),
            Some(String::from("Bluey")),
        ))
        .id();

    let std_cube_black_vessel = commands
        .spawn((
            Transform::from_xyz(0.0, 2.0, 1.0),
            PhysicsBodyType::Controlled,
            Mesh3d(asset_server.load::<Mesh>("meshes/cube.obj")),
            MeshMaterial3d(materials.add(Color::BLACK)),
            DebugShowAxes,
            DebugCameraPoint,
            DebugCollisionMesh,
        ))
        .insert(VesselBundle::new(
            String::from("Std black cube"),
            Some(String::from("Blacky")),
        ))
        .id();

    commands.spawn(Player).insert(SoulBundle::new(
        cube_vessel,
        donut_vessel,
        vec![
            cube_vessel,
            donut_vessel,
            std_cube_vessel,
            std_cube_black_vessel,
        ],
    ));
}
