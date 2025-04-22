use bevy::prelude::*;
use crate::gameplay::player::camera::CameraOrder;

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
    ));
}
