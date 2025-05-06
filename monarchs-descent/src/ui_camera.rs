use crate::CameraOrder;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_ui_camera);
}

fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("UI Camera"),
        Camera2d,
        IsDefaultUiCamera,
        Camera {
            order: CameraOrder::Ui.into(),
            ..default()
        },
        RenderLayers::layer(1),
    ));
}
