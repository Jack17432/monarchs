use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EguiPlugin {
        enable_multipass_for_primary_context: true,
    })
    .add_plugins(DefaultInspectorConfigPlugin);
}
