use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .insert_resource(Gravity(Vec3::NEG_Y * 9.8));

    app.insert_gizmo_config(
        PhysicsGizmos::default(),
        GizmoConfig {
            enabled: true,
            ..default()
        },
    );
}
