use bevy::prelude::*;

#[derive(Component)]
pub struct ShowAxes;

pub struct DebugTools;

impl Plugin for DebugTools {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_axes);
    }
}

fn draw_axes(mut gizmos: Gizmos, query: Query<&Transform, With<ShowAxes>>) {
    for &transform in &query {
        let length = 2.0;
        gizmos.axes(transform, length);
    }
}
