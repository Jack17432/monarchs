mod third_party;

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
    }
}
