use crate::LookDirection;
use bevy::color::palettes::basic::BLUE;
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

fn draw_axes(mut gizmos: Gizmos, query: Query<&Transform, With<DebugShowAxes>>) {
    for &transform in &query {
        let length = 2.0;
        gizmos.axes(transform, length);
    }
}

fn draw_looking_direction(
    mut gizmos: Gizmos,
    query: Query<(&LookDirection, &Transform), With<DebugShowLookingDir>>,
) {
    for (direction, transform) in query.iter() {
        let forward = (direction.0 * Vec3::X).normalize();
        gizmos.arrow(transform.translation, transform.translation + forward, BLUE);
    }
}
