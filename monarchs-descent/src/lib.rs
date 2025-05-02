mod gameplay;
mod third_party;
mod ui_camera;

use bevy::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Monarchs descent".to_string(),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }));

        app.add_plugins(third_party::plugin);

        app.add_plugins(gameplay::plugin)
            .add_plugins(ui_camera::plugin);
    }
}

enum CameraOrder {
    World,
    ViewModel,
    Ui,
}

impl From<CameraOrder> for isize {
    fn from(order: CameraOrder) -> Self {
        order as isize
    }
}
