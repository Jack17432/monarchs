mod console;
mod egui_plugin;
mod enhanced_input;
mod fps;
mod physics;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        fps::plugin,
        physics::plugin,
        egui_plugin::plugin,
        enhanced_input::plugin,
        console::plugin,
    ));
}
