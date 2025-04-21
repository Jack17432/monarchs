mod ui;
mod worlds;
pub mod camera;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((worlds::plugin, ui::plugin));
}
