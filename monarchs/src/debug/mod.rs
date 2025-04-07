use crate::core::physics::Collider;
use crate::views::player_camera::PlayerCameraInfo;
use bevy::color::palettes::basic::WHITE;
use bevy::color::palettes::css::PINK;
use bevy::prelude::*;
use bevy::transform::systems::propagate_transforms;

#[derive(Component, Debug)]
pub struct DebugShowAxes;

#[derive(Component, Debug)]
pub struct DebugCameraPoint;

#[derive(Component, Debug)]
pub struct DebugCollisionMesh;

pub struct DebugToolsPlugin;

impl Plugin for DebugToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (draw_axes, draw_camera_point, draw_collision_mesh).after(propagate_transforms),
        );
    }
}

fn draw_axes(mut gizmos: Gizmos, query: Query<&GlobalTransform, With<DebugShowAxes>>) {
    for &global_transform in &query {
        let length = 2.0;
        gizmos.axes(global_transform, length);
    }
}

fn draw_camera_point(
    mut gizmos: Gizmos,
    query: Query<(&PlayerCameraInfo, &GlobalTransform), With<DebugCameraPoint>>,
) {
    for (direction, global_transform) in query.iter() {
        let translation = global_transform.translation();
        let forward = (direction.0 * Vec3::X).normalize();
        gizmos.arrow(translation, translation + forward, PINK);
    }
}

fn draw_collision_mesh(
    mut gizmos: Gizmos,
    query: Query<(&GlobalTransform, &Collider), With<DebugCollisionMesh>>,
) {
    for (global_transform, collider) in query.iter() {
        let translation = global_transform.translation();
    }
}
