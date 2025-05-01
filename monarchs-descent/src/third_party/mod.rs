mod egui_plugin;
mod enhanced_input;
mod framepace;
mod physics;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        framepace::plugin,
        physics::plugin,
        egui_plugin::plugin,
        enhanced_input::plugin,
    ));
}
