use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_framepace::FramepacePlugin;
use bevy_obj::ObjPlugin;
use monarchs::config::ConfigPlugin;
use monarchs::controllers::player::{Player, PlayerControlled};
use monarchs::controllers::ControllerPluginGroup;
use monarchs::core::{solver, Collider, PhysicsBodyType};
use monarchs::debug::*;
use monarchs::environment::*;
use monarchs::views::player_camera::*;
use monarchs::views::*;
use monarchs::void_born::souls::SoulBundle;
use monarchs::void_born::vessels::VesselBundle;
use monarchs::void_born::VoidBornPlugin;
use monarchs::{ui, GameState};

fn main() {
    App::new()
        // .insert_resource(Time::<Fixed>::from_hz(100.0))
        .add_plugins((DefaultPlugins, EguiPlugin, FramepacePlugin, ObjPlugin))
        .add_plugins(DebugTools)
        .add_plugins(ConfigPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(ViewsPluginGroup)
        .add_plugins(VoidBornPlugin)
        .add_plugins(ControllerPluginGroup)
        .add_plugins(solver::PhysicsSolverPlugin)
        .add_plugins(ui::UiPlugin)
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
            Collider::from_capsule(Vec3::new(0.0, 0.0, -0.5), Vec3::new(0.0, 0.0, 0.5), 1.0),
            PlayerControlled,
            PlayerCameraInfo(Quat::IDENTITY),
            Mesh3d(asset_server.load::<Mesh>("meshes/cube.obj")),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("textures/Grass.png")),
                ..default()
            })),
            DebugShowAxes,
            DebugCameraPoint,
        ))
        .insert(VesselBundle::new(
            String::from("Grassius"),
            Some(String::from("Grass block")),
        ))
        .id();

    let donut_vessel = commands
        .spawn((
            Transform::from_xyz(0.0, 0.0, 1.0),
            PhysicsBodyType::Controlled,
            Collider::from_capsule(Vec3::new(0.0, 0.0, 0.1), Vec3::new(0.0, 0.0, -0.1), 0.3),
            SceneRoot(asset_server.load("meshes/donut.glb#Scene0")),
            DebugShowAxes,
            DebugCameraPoint,
        ))
        .insert(VesselBundle::new(String::from("Donut"), None))
        .id();

    let std_cube_vessel = commands
        .spawn((
            Transform::from_xyz(2.0, 0.0, 1.0),
            PhysicsBodyType::Controlled,
            Collider::from_capsule(Vec3::new(0.0, 0.0, -0.5), Vec3::new(0.0, 0.0, 0.5), 1.0),
            Mesh3d(asset_server.load::<Mesh>("meshes/cube.obj")),
            MeshMaterial3d(materials.add(Color::WHITE)),
            DebugShowAxes,
            DebugCameraPoint,
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
            Collider::from_capsule(Vec3::new(0.0, 0.0, -0.5), Vec3::new(0.0, 0.0, 0.5), 1.0),
            Mesh3d(asset_server.load::<Mesh>("meshes/cube.obj")),
            MeshMaterial3d(materials.add(Color::BLACK)),
            DebugShowAxes,
            DebugCameraPoint,
        ))
        .insert(VesselBundle::new(
            String::from("Std black cube"),
            Some(String::from("Blacky")),
        ))
        .id();

    commands.spawn(Player).insert(SoulBundle::new(
        cube_vessel,
        donut_vessel,
        vec![cube_vessel, donut_vessel, std_cube_vessel, std_cube_black_vessel],
    ));
}
