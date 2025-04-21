pub mod files;
pub mod saves;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((files::plugin, saves::plugin));
}