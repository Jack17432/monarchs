use crate::LookDirection;
use bevy::color::palettes::basic::BLUE;
use bevy::color::palettes::css::PINK;
use bevy::prelude::*;

#[derive(Component)]
pub struct DebugShowAxes;

#[derive(Component)]
pub struct DebugShowLookingDir;

pub struct DebugTools;

impl Plugin for DebugTools {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (draw_axes, draw_looking_direction));
    }
}

fn draw_axes(mut gizmos: Gizmos, query: Query<(Entity, &GlobalTransform, &Transform), With<DebugShowAxes>>) {
    for (entity, &global_transform, &transform) in &query {
        info!(entity =? entity, global_transform = ?global_transform, transform =? transform);
        let length = 2.0;
        gizmos.axes(global_transform, length);
    }
}

fn draw_looking_direction(
    mut gizmos: Gizmos,
    query: Query<(&LookDirection, &GlobalTransform), With<DebugShowLookingDir>>,
) {
    for (direction, global_transform) in query.iter() {
        let forward = (direction.0 * Vec3::X).normalize();
        gizmos.arrow(global_transform.translation(), global_transform.translation() + forward, PINK);
    }
}
