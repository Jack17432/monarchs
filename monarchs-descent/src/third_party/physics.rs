use avian3d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .insert_resource(Gravity(Vec3::NEG_Y * 25.0));

    app.insert_gizmo_config(
        PhysicsGizmos::default(),
        GizmoConfig {
            enabled: true,
            ..default()
        },
    );
}
