mod egui_plugin;
mod framepace_plugin;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((egui_plugin::plugin, framepace_plugin::plugin));
}
